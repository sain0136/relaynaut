# Building Code

To build your Rust project, use the following command at the root of workspace all members will be built:

```sh
cargo build
```

If building fails try to run cargo clean first:

```sh
cargo clean
```

# Linting and Formatting Standards

## Linting with Clippy

To enforce strict linting across all targets and features, treating warnings as errors, use the following command:

```sh
cargo clippy --all-targets --all-features -- -D warnings
```

### Custom Lint Command

You can configure `.cargo/config.toml` to create a custom lint command. This allows you to run:

```sh
cargo lint
```

which will execute the strict clippy linting as defined above.

## Code Formatting

To ensure consistent code formatting, use:

```sh
cargo fmt --all -- --check
```

The `--check` flag is recommended for CI pipelines to fail the build if formatting does not meet project standards.

---

**Summary:**

- Run `cargo lint` for linting.
- Run `cargo fmt` for formatting.
- Both commands enforce elite code quality and consistency.
