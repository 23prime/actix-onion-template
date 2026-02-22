# actix-onion-template

A template for building web applications using Actix Web and the Onion architecture.

## Development

### Pre-requirements

- [mise](https://mise.jdx.dev)
- [rustup](https://rustup.rs)

### Setup

```bash
mise run setup
```

### Commands

| Command                  | Alias         | Description                        |
| ------------------------ | ------------  | ---------------------------------- |
| `mise run setup`         | `mise run s`  | Install tools and set up git hooks |
| `mise run fix`           | `mise run f`  | Auto-fix all issues                |
| `mise run check`         | `mise run c`  | Check for all issues               |
| `mise run fix-and-check` | `mise run fc` | Fix and then check                 |

#### Rust

| Command                     | Alias         | Description                    |
| --------------------------- | ------------- | ------------------------------ |
| `mise run rs-run`           | `mise run rr` | Run the application            |
| `mise run rs-fix`           | `mise run rf` | Fix Rust code (clippy + fmt)   |
| `mise run rs-check`         | `mise run rc` | Check Rust code                |
| `mise run rs-build`         |               | Build the application          |
| `mise run rs-build-release` |               | Build in release mode          |
| `mise run rs-clean`         |               | Clean build artifacts          |

## Contributing

See [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md).

## License

[MIT](LICENSE)
