# Contributing to apalis-board

Thank you for your interest in contributing to apalis-board! This document provides guidelines and information about our development process.

For detailed information about our CI/CD infrastructure, see [CI/CD Infrastructure](.github/CI_CD_INFRASTRUCTURE.md).

## Development Setup

### Prerequisites

- **Rust**: Install from [rustup.rs](https://rustup.rs/)
- **Node.js**: Version 20 or higher
- **Trunk**: Install with `cargo install trunk`
- **wasm32 target**: Install with `rustup target add wasm32-unknown-unknown`

### Getting Started

1. Clone the repository:
   ```bash
   git clone https://github.com/apalis-dev/apalis-board.git
   cd apalis-board
   ```

2. Install dependencies:
   ```bash
   npm ci  # Install Tailwind CSS dependencies
   cargo build --workspace  # Build all crates
   ```

3. Build the frontend:
   ```bash
   cd crates/board
   trunk build
   ```

## Project Structure

```
apalis-board/
├── crates/
│   ├── api/          # Backend API for actix-web and axum
│   ├── board/        # Frontend Leptos application
│   └── types/        # Shared types between frontend and backend
├── examples/         # Example implementations (not part of workspace)
└── .github/
    └── workflows/    # CI/CD workflows
```

## CI/CD Workflows

We use GitHub Actions for continuous integration and deployment:

### Main Workflows

1. **All Checks (`ci.yml`)**: Combined workflow that runs all checks
   - Rust build, test, format, and clippy
   - Frontend build
   - Security checks

2. **Rust CI (`rust-ci.yml`)**: Comprehensive Rust checks
   - Check compilation for all crates
   - Run tests
   - Format checking with rustfmt
   - Linting with clippy
   - Feature combination testing

3. **Frontend Build (`frontend-build.yml`)**: Build the Leptos frontend
   - Compiles WASM
   - Builds with Trunk
   - Runs Tailwind CSS

4. **Release (`release.yml`)**: Automated releases on version tags
   - Creates GitHub releases
   - Builds and uploads frontend artifacts
   - Packages Rust crates

5. **Security Audit (`security.yml`)**: Security scanning
   - cargo-audit for vulnerability scanning
   - cargo-deny for license and supply chain checks

6. **Code Coverage (`coverage.yml`)**: Test coverage reporting
   - Generates coverage with tarpaulin
   - Uploads to Codecov

7. **Documentation Check (`docs.yml`)**: Documentation validation
   - Markdown link checking
   - rustdoc build

### Automated Maintenance

- **Dependabot**: Automatically creates PRs for dependency updates
  - Rust dependencies (weekly)
  - npm dependencies (weekly)
  - GitHub Actions (weekly)

## Development Workflow

### Before Committing

1. **Format your code**:
   ```bash
   cargo fmt --all
   ```

2. **Run clippy**:
   ```bash
   cargo clippy --workspace --all-features
   ```

3. **Run tests**:
   ```bash
   cargo test --workspace
   ```

4. **Build the frontend**:
   ```bash
   cd crates/board
   trunk build
   ```

### Making a Pull Request

1. Create a new branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes and commit:
   ```bash
   git add .
   git commit -m "Description of your changes"
   ```

3. Push your branch:
   ```bash
   git push origin feature/your-feature-name
   ```

4. Open a Pull Request on GitHub

### PR Requirements

All PRs must pass the following checks:
- ✅ Rust compilation
- ✅ Tests passing
- ✅ Code formatting (rustfmt)
- ✅ Clippy lints
- ✅ Frontend builds successfully
- ✅ No security vulnerabilities

## Testing

### Running Tests

```bash
# Test all crates
cargo test --workspace

# Test a specific crate
cargo test -p apalis-board-types
cargo test -p apalis-board-api --all-features
```

### Coverage

To generate code coverage locally:

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage
cargo tarpaulin --out html --output-dir coverage
```

## Code Style

- Follow Rust standard style (enforced by rustfmt)
- Use meaningful variable and function names
- Add comments for complex logic
- Write documentation for public APIs

## Feature Flags

The API crate supports optional framework integrations:
- `actix`: Support for actix-web
- `axum`: Support for axum

When adding code that depends on these features, use appropriate cfg attributes:

```rust
#[cfg(feature = "actix")]
// actix-specific code

#[cfg(feature = "axum")]
// axum-specific code
```

## Release Process

Releases are automated through GitHub Actions:

1. Update version in `Cargo.toml` files
2. Update `CHANGELOG.md`
3. Create and push a version tag:
   ```bash
   git tag -a v1.0.0 -m "Release v1.0.0"
   git push origin v1.0.0
   ```
4. GitHub Actions will automatically:
   - Create a GitHub release
   - Build and upload artifacts
   - Package crates

## Getting Help

- Open an issue for bugs or feature requests
- Start a discussion for questions
- Check existing issues and PRs

## Code of Conduct

Please note that this project is released with a [Contributor Code of Conduct](CODE_OF_CONDUCT.md). By participating in this project you agree to abide by its terms.

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (see LICENSE file).
