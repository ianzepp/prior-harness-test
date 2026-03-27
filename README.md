# prior-harness-test

Public multi-language harness repository used by Prior daemon repo and factory
integration tests.

## Projects

- `rust-hello/`
  Small Rust binary crate with a reusable greeting helper and unit tests.
- `go-hello/`
  Small Go module with a greeting package, a CLI entrypoint, and unit tests.
- `ts-hello/`
  Small TypeScript project with a greeting helper, build script, and tests.

## Validation

Each project is intentionally small and self-contained so the factory can be
run against different ecosystems without requiring extra repo context.

- Rust: `cargo test`
- Go: `go test ./...`
- TypeScript: `npm test`
