# AGENTS.md

This file provides guidance to AI coding agents when working with code in this repository.

## General agent rules

- When users ask questions, answer them instead of doing the work.

### Shell Rules

- Always use `rm -f` (never bare `rm`)
- Run `git` commands in the current directory (do not use the `-C` option)

## Project Overview

A Rust web application template built with [Actix Web](https://actix.rs) and the Onion architecture.
The application code lives in the `app/` directory.
Development tooling (linting, formatting, spell checking, git hooks) is managed via [mise](https://mise.jdx.dev).

## Development Rules

### Use mise Tasks

Prefer `mise run <task>` over running underlying tools directly.

Key tasks:

- `mise run setup` — first-time setup (installs tools and git hooks)
- `mise run rs-run` — run the application
- `mise run rs-fix` — auto-fix Rust code (clippy + fmt)
- `mise run rs-check` — check Rust code (clippy + fmt + tests)

### Fix and Check After Editing Files

After editing any file, run the following to auto-fix and verify:

```bash
mise run fix
mise run check
```

For Rust files specifically:

```bash
mise run rs-fix
mise run rs-check
```
