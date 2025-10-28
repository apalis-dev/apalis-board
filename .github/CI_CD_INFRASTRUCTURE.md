# CI/CD Infrastructure

This document provides an overview of the CI/CD infrastructure for apalis-board.

## Overview

The apalis-board repository uses GitHub Actions for continuous integration, deployment, and automated maintenance. The infrastructure is designed to ensure code quality, security, and reliability.

## Workflows

### Core CI Workflows

#### 1. All Checks (`ci.yml`)
**Trigger**: Push to master/main, Pull Requests  
**Purpose**: Combined workflow that serves as the main status check

This workflow runs:
- Rust compilation checks
- Test suite
- Code formatting validation
- Clippy linting
- Frontend build
- Security audit

**Status**: This is the primary status check for PRs

#### 2. Rust CI (`rust-ci.yml`)
**Trigger**: Push to master/main, Pull Requests  
**Purpose**: Comprehensive Rust quality checks

Jobs:
- **check**: Verifies all crates compile
- **test**: Runs test suites for types and api crates
- **fmt**: Validates code formatting with rustfmt
- **clippy**: Lints code with clippy
- **features**: Tests feature combinations (actix, axum)

#### 3. Frontend Build (`frontend-build.yml`)
**Trigger**: Push to master/main, Pull Requests  
**Purpose**: Builds the Leptos WASM frontend

Steps:
1. Install Rust with wasm32-unknown-unknown target
2. Install Trunk (WASM bundler)
3. Install npm dependencies (Tailwind CSS)
4. Build frontend with Trunk
5. Upload build artifacts

### Release Workflow

#### 4. Release (`release.yml`)
**Trigger**: Version tags (v*.*.*)  
**Purpose**: Automated release creation and artifact publishing

The workflow:
1. Creates a GitHub release
2. Builds frontend and packages as tar.gz
3. Builds and packages Rust crates
4. Uploads all artifacts to the release

**Usage**: Push a version tag to trigger
```bash
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

### Security and Quality

#### 5. Security Audit (`security.yml`)
**Trigger**: 
- Push/PR on Cargo.toml or Cargo.lock changes
- Weekly schedule (Mondays at 9 AM UTC)

Jobs:
- **audit**: Scans for security vulnerabilities with cargo-audit
- **deny**: Checks dependencies with cargo-deny for:
  - Known security advisories
  - License compliance
  - Banned/yanked crates
  - Multiple versions of same crate

#### 6. Code Coverage (`coverage.yml`)
**Trigger**: Push to master/main, Pull Requests  
**Purpose**: Generates and reports code coverage

Uses:
- cargo-tarpaulin for coverage generation
- Codecov for coverage reporting and tracking

#### 7. Documentation Check (`docs.yml`)
**Trigger**: 
- Push/PR to master/main
- Weekly schedule (Sundays at midnight UTC)

Jobs:
- **link-check**: Validates markdown links
- **rustdoc**: Builds Rust documentation with warnings as errors

### Maintenance

#### 8. Tailwind CSS Check (`nodejs.yml`)
**Trigger**: Push to master/main, Pull Requests  
**Purpose**: Validates npm dependencies

Checks:
- Installs npm dependencies
- Checks for outdated packages
- Runs security audit

#### 9. Cache Warmup (`cache-warmup.yml`)
**Trigger**: 
- Weekly schedule (Sundays at midnight UTC)
- Manual workflow dispatch

**Purpose**: Pre-warms caches to improve CI performance

Jobs:
- Builds all Rust crates to populate cargo cache
- Installs npm dependencies to populate npm cache

#### 10. Dependabot Configuration (`dependabot.yml`)
**Schedule**: Weekly (Mondays)  
**Purpose**: Automated dependency updates

Monitors:
- Cargo dependencies (Rust)
- npm dependencies (Tailwind CSS)
- GitHub Actions versions

Creates PRs for updates grouped by ecosystem.

## Caching Strategy

The CI uses multiple caching strategies:

1. **Rust Cache** (Swatinem/rust-cache@v2)
   - Caches compiled dependencies
   - Shared across workflows with `shared-key`
   - Significantly reduces build times

2. **npm Cache**
   - Built into setup-node action
   - Caches node_modules dependencies

3. **Weekly Warmup**
   - Refreshes caches weekly
   - Ensures CI has latest pre-compiled dependencies

## Security Features

1. **cargo-audit**: Checks for known security vulnerabilities
2. **cargo-deny**: Enforces security and licensing policies
3. **Dependabot**: Automated security updates
4. **npm audit**: Checks npm dependencies for vulnerabilities

## Status Badges

The README includes badges for:
- All Checks status
- Rust CI status
- Frontend Build status
- Security Audit status
- Code Coverage percentage

## Troubleshooting

### Workflow Failures

**Rust CI fails on formatting**:
```bash
cargo fmt --all
```

**Rust CI fails on clippy**:
```bash
cargo clippy --workspace --all-features
```

**Frontend build fails**:
```bash
cd crates/board
trunk build
```

**Tests fail**:
```bash
cargo test --workspace
```

### Common Issues

1. **Workspace member not found**: Examples are excluded from workspace as they depend on external repos
2. **Cache issues**: Manually trigger cache-warmup workflow or wait for weekly refresh
3. **Outdated dependencies**: Wait for Dependabot PRs or run `cargo update` / `npm update`

## Manual Triggers

Some workflows can be triggered manually:

1. **Cache Warmup**: Go to Actions → Cache Warmup → Run workflow
2. **Release**: Push a version tag (preferred over manual trigger)

## Contributing

When contributing, ensure:
- All CI checks pass before requesting review
- Follow the PR template
- Update documentation if needed
- Add tests for new features

For more details, see [CONTRIBUTING.md](../CONTRIBUTING.md).

## Future Improvements

Potential enhancements:
- [ ] Nightly builds for testing against Rust nightly
- [ ] Performance benchmarking
- [ ] Automated changelog generation
- [ ] Deploy previews for frontend changes
- [ ] Integration tests with example services
