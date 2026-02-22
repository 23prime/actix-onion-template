# actix-onion-template

A template for building web applications using Actix Web and the Onion architecture.

## Architecture

This template follows the Onion architecture, where dependencies flow strictly inward.

```mermaid
graph TB
    subgraph outer["Outer Layer"]
        server
        container
    end
    subgraph middle["Infrastructure / Presentation Layer"]
        infrastructure
        presentation
    end
    subgraph app["Application Layer"]
        use_case
    end
    subgraph core["Domain Layer (Core)"]
        domain
    end

    server --> container
    server --> infrastructure
    server --> presentation
    container --> domain
    infrastructure --> domain
    presentation --> container
    presentation --> use_case
    presentation --> domain
    use_case --> domain
```

| Crate            | Kind   | Responsibility                                     |
| ---------------- | ------ | -------------------------------------------------- |
| `domain`         | lib    | Entities, value objects, repository traits         |
| `use_case`       | lib    | Application services, business logic               |
| `presentation`   | lib    | HTTP handlers, routing                             |
| `infrastructure` | lib    | Repository implementations, external services      |
| `container`      | lib    | DI container holding repository trait objects      |
| `server`         | binary | Wires dependencies, initializes infrastructure     |

## Tech Stack

| Category  | Technology                                                   |
| --------- | ------------------------------------------------------------ |
| Framework | [Actix Web](https://actix.rs) 4                              |
| Database  | PostgreSQL                                                   |
| ORM / SQL | [sqlx](https://github.com/launchbadge/sqlx) 0.8              |
| Migration | [Atlas](https://atlasgo.io) (declarative schema management)  |
| Runtime   | [Tokio](https://tokio.rs)                                    |
| Tooling   | [mise](https://mise.jdx.dev)                                 |
| Testing   | [runn](https://github.com/k1LoW/runn)                        |

## Development

### Pre-requirements

- [mise](https://mise.jdx.dev)
- [rustup](https://rustup.rs)
- [Docker](https://www.docker.com) (with Compose)

### Setup

```bash
mise run setup
mise run db-up-d
mise run db-apply
```

### Commands

| Command                  | Alias         | Description                        |
| ------------------------ | ------------- | ---------------------------------- |
| `mise run dev`           | `mise run d`  | Start all development services     |
| `mise run setup`         | `mise run s`  | Install tools and set up git hooks |
| `mise run fix`           | `mise run f`  | Auto-fix all issues                |
| `mise run check`         | `mise run c`  | Check for all issues               |
| `mise run fix-and-check` | `mise run fc` | Fix and then check                 |

#### Rust

| Command                     | Alias         | Description                     |
| --------------------------- | ------------- | ------------------------------- |
| `mise run rs-run`           | `mise run rr` | Run the application             |
| `mise run rs-watch`         | `mise run rw` | Run with hot reload (watchexec) |
| `mise run rs-fix`           | `mise run rf` | Fix Rust code (clippy + fmt)    |
| `mise run rs-check`         | `mise run rc` | Check Rust code                 |
| `mise run rs-build`         |               | Build the application           |
| `mise run rs-build-release` |               | Build in release mode           |
| `mise run rs-clean`         |               | Clean build artifacts           |

#### Database

| Command              | Description                             |
| -------------------- | --------------------------------------- |
| `mise run db-up-d`   | Start the database container (detached) |
| `mise run db-up`     | Start the database container (attached) |
| `mise run db-stop`   | Stop the database container             |
| `mise run db-down`   | Stop and remove the database container  |
| `mise run db-logs`   | Follow database container logs          |
| `mise run db-apply`  | Apply schema to the database            |

#### Testing

| Command                      | Alias          | Description             |
| ---------------------------- | -------------- | ----------------------- |
| `mise run integration-test`  | `mise run it`  | Run integration tests   |

## Contributing

See [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md).

## License

[MIT](LICENSE)
