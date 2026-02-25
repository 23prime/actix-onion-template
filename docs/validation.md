# Validation Guideline

This document defines where validation belongs in the onion architecture,
establishes `garde` as the chosen validation library, and provides a
repeatable pattern for contributors to follow.

## Layer Responsibilities

| Layer | Responsibility |
| --- | --- |
| Presentation | HTTP structural validity — handled by `serde` (type errors, missing required fields) |
| Use Case | Business rule validation — empty strings, length limits, format rules via `garde` |
| Domain | Invariants that must always hold — enforced in constructors / value objects |

Validation must not leak across layers:

- Presentation must not know about business rules
- Domain must not call `garde`; it uses typed constructors that return `Result`

## Validation Library: garde

The project uses [`garde`](https://crates.io/crates/garde) for use-case-layer validation.

Add to the use case crate:

```toml
# use_case/Cargo.toml
garde = { workspace = true }
```

Add to the workspace:

```toml
# Cargo.toml (workspace)
garde = { version = "0.22", features = ["derive"] }
```

## Pattern for Input Structs

Derive `garde::Validate` on Input structs and annotate each field:

```rust
use garde::Validate;

#[derive(Validate)]
pub struct CreateUserInput {
    #[garde(length(min = 1))]
    pub name: String,
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 8))]
    pub password: String,
}
```

## Calling Validation in `execute()`

At the very start of `execute()`, call `input.validate()` and map the error:

```rust
pub async fn execute(&self, input: CreateUserInput) -> Result<User, CreateUserError> {
    input.validate().map_err(CreateUserError::Validation)?;
    // ...
}
```

## Error Enum Convention

Add a `Validation` variant to the use-case error enum, wrapping `garde::Report`:

```rust
#[derive(Debug)]
pub enum CreateUserError {
    Validation(garde::Report),
    User(UserError),
    Auth(AuthError),
}
```

For use cases that previously returned a domain error type directly (e.g. `Login`),
introduce a dedicated error enum:

```rust
#[derive(Debug)]
pub enum LoginError {
    Validation(garde::Report),
    InvalidCredentials,
    Unexpected(String),
}
```

## HTTP Response for Validation Errors

Return `422 Unprocessable Entity` with:

```json
{
  "error": "validation_error",
  "fields": {
    "email": ["length is lower than 1"],
    "password": ["length is lower than 8"]
  }
}
```

Build the `fields` map from `garde::Report` using a shared helper in the presentation crate,
then return it in the handler's `Validation` arm:

```rust
// presentation/src/lib.rs
pub(crate) fn validation_fields(
    report: &garde::Report,
) -> std::collections::HashMap<String, Vec<String>> {
    let mut fields: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    for (path, error) in report.iter() {
        fields
            .entry(path.to_string())
            .or_default()
            .push(error.message().to_string());
    }
    fields
}
```

```rust
// handler match arm
Err(CreateUserError::Validation(report)) => {
    HttpResponse::UnprocessableEntity().json(serde_json::json!({
        "error": "validation_error",
        "fields": validation_fields(&report),
    }))
}
```

Note: `garde::Report::iter()` yields `&(Path, Error)` pairs — one entry per violation.
Multiple violations on the same field produce multiple entries with the same path,
which the helper groups into a `Vec<String>` under that field key.

Note: `impl actix_web::ResponseError` cannot be used here because both `LoginError`/`CreateUserError`
(from `use_case`) and `ResponseError` (from `actix_web`) are foreign to the `presentation` crate.
The helper-function pattern achieves the same deduplication.

## Step-by-Step: Adding a New Rule

1. Add or update a `#[garde(...)]` attribute on the Input struct field
2. No changes are needed in other layers unless the rule changes which variants the error enum must cover
3. Add or update a unit test in the use-case crate that triggers the new rule and asserts the returned error is `Validation`
4. Run `mise run rs-check` to confirm everything compiles and tests pass

## Domain Invariants

The domain layer enforces invariants that must always hold, regardless of caller.
Use typed constructors that return `Result` rather than exposing plain field assignment.

`Credentials::new` in `domain/src/credentials.rs` is an existing example — it hashes
the password and returns `Err(AuthError::Unexpected(...))` if the hashing operation itself
fails. This is a domain invariant: a `Credentials` value can never exist without a hashed password.

To add a structural invariant such as "email must contain `@`", introduce a value object:

```rust
pub struct Email(String);

impl Email {
    pub fn new(raw: &str) -> Result<Self, UserError> {
        if !raw.contains('@') {
            return Err(UserError::InvalidEmail);
        }
        Ok(Self(raw.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
```

Domain invariants must not depend on `garde`.
The constructor returns a typed error from the domain crate.

The use-case layer's `garde` validation runs *before* domain constructors are called,
so format checks (e.g. `#[garde(email)]`) live in the use case and
structural checks (e.g. "cannot construct this value in an invalid state") live in the domain.

## Consolidating Error Mapping in the Presentation Layer

Rather than repeating the `garde::Report → JSON` conversion in every handler,
implement `actix_web::ResponseError` for each use-case error enum.
This keeps handler bodies free of error-formatting logic.

```rust
use actix_web::HttpResponse;
use std::collections::HashMap;

impl actix_web::ResponseError for CreateUserError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateUserError::Validation(report) => {
                let fields: HashMap<_, _> = report
                    .iter()
                    .map(|(path, errors)| {
                        let msgs: Vec<_> =
                            errors.iter().map(|e| e.message().to_string()).collect();
                        (path.to_string(), msgs)
                    })
                    .collect();
                HttpResponse::UnprocessableEntity().json(serde_json::json!({
                    "error": "validation_error",
                    "fields": fields,
                }))
            }
            CreateUserError::User(domain::user::UserError::EmailAlreadyExists) => {
                HttpResponse::Conflict()
                    .json(serde_json::json!({ "error": "email_already_exists" }))
            }
            _ => HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "internal_server_error" })),
        }
    }
}
```

With `ResponseError` implemented, the handler returns `Result<HttpResponse, CreateUserError>`
and Actix Web calls `error_response()` automatically on `Err`:

```rust
pub async fn create_user(
    container: web::Data<Container>,
    body: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, CreateUserError> {
    let use_case = CreateUser::new(
        container.user_repo.clone(),
        container.credentials_repo.clone(),
    );
    let input = CreateUserInput {
        name: body.name.clone(),
        email: body.email.clone(),
        password: body.password.clone(),
    };
    let user = use_case.execute(input).await?;
    Ok(HttpResponse::Created().json(UserResponse {
        id: user.id.0.to_string(),
        name: user.name,
        email: user.email,
        created_at: user.created_at.to_rfc3339(),
    }))
}
```
