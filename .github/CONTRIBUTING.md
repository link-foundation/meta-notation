# Contributing to Meta-Notation

Thank you for your interest in contributing to meta-notation!

## Development Setup

### JavaScript/TypeScript

```bash
cd js
npm install
npm run build
npm test
```

### Rust

```bash
cd rust
cargo build
cargo test
```

## Running Tests

### JavaScript
```bash
cd js
npm test                  # Run all tests
npm run build            # Build the project
```

### Rust
```bash
cd rust
cargo test               # Run all tests
cargo test --doc         # Run doc tests
cargo fmt                # Format code
cargo clippy             # Lint code
```

## Continuous Integration

We use GitHub Actions for CI/CD:

- **CI/CD**: Runs linting and tests on multiple platforms
- **Manual Release**: Trigger releases via GitHub Actions
- **Auto-publish**: Automatic publishing to npm and crates.io on release

## Making Changes

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Ensure tests pass: `npm test` (JS) and `cargo test` (Rust)
5. Run formatters: `cargo fmt` (Rust)
6. Submit a pull request

## Releasing

We use [changesets](https://github.com/changesets/changesets) for version management.

### Manual Release
1. Go to GitHub Actions
2. Select "Manual Release" workflow
3. Choose version bump type (patch/minor/major)
4. Review and merge the created PR
5. The release will be automated

## Code Style

### JavaScript/TypeScript
- Use test-anywhere for tests
- Follow existing code patterns
- Keep tests in `tests/` folder

### Rust
- Follow standard Rust conventions
- Run `cargo fmt` before committing
- Run `cargo clippy` to catch common mistakes
- Add tests for new functionality

## License

By contributing, you agree to license your contribution under the Unlicense (public domain).
