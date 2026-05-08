# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0-rc.8] - 2026-05-08

### Changed

- Allow static queue handling ([#110](https://github.com/apalis-dev/apalis-board/pull/110))
- chore: update API path handling and improve HTML base tag ([#108](https://github.com/apalis-dev/apalis-board/pull/108))

---

## [1.0.0-rc.7] - 2026-04-11

### Changed

- Make `ui` feature opt-in and reduce default compile footprint ([#95](https://github.com/apalis-dev/apalis-board/pull/95))
- Centralize shared dependencies in `[workspace.dependencies]` ([#96](https://github.com/apalis-dev/apalis-board/pull/96))
- Release `v1.0.0-rc.7` ([#97](https://github.com/apalis-dev/apalis-board/pull/97))

---

## [1.0.0-rc.6]

### Fixed

- Humanize home stats timestamps ([#67](https://github.com/apalis-dev/apalis-board/pull/67))
- Correct task pagination row count for status-filtered views ([#76](https://github.com/apalis-dev/apalis-board/pull/76))

### Changed

- Release `v1.0.0-rc.6` ([#77](https://github.com/apalis-dev/apalis-board/pull/77))

---

## [1.0.0-rc.5]

### Changed

- Refactor to remove path normalization ([#55](https://github.com/apalis-dev/apalis-board/pull/55))

### Dependencies

- Bump `leptos-struct-table` from `0.16.0` to `0.17.0` ([#48](https://github.com/apalis-dev/apalis-board/pull/48))

---

## [1.0.0-rc.4]

### Added

- Include distribution assets in crate for bundling ([#22](https://github.com/apalis-dev/apalis-board/pull/22))
- Include locales in crate ([#23](https://github.com/apalis-dev/apalis-board/pull/23))

### Changed

- Rewrite the board in Rust
- Upgrade to the latest `apalis` version
- Release `v1.0.0-rc.4` ([#66](https://github.com/apalis-dev/apalis-board/pull/66))

---

## [1.0.0-rc.3]

### Dependencies

- Update dependencies for `v1.0.0-rc.1`

### Changed

- Release `v1.0.0-rc.3` ([#56](https://github.com/apalis-dev/apalis-board/pull/56))

---

## [1.0.0-rc.2]

### Changed

- Release `v1.0.0-rc.2` ([#45](https://github.com/apalis-dev/apalis-board/pull/45))

---

## [1.0.0-rc.1]

### Changed

- Initial release candidate
- Rewrite the board in Rust
- Upgrade to the latest `apalis` version

---

## [1.0.0-beta.1]

### Changed

- Release `v1.0.0-beta.1` ([#26](https://github.com/apalis-dev/apalis-board/pull/26))
- Follow-up beta release adjustments ([#27](https://github.com/apalis-dev/apalis-board/pull/27))

---

## [1.0.0-alpha.2]

### Changed

- Release `v1.0.0-alpha.2` ([#25](https://github.com/apalis-dev/apalis-board/pull/25))

### CI/CD

- Streamline workflows
- Include lockfiles
- Add `yarn.lock`
- Minor workflow fixes
- Fix dependencies for CI and changelog generation
- Improve relative path handling
- Add checkout sources
- Add toolchain, targets, and caching
- Add Yarn and `setup-node`
- Remove nightly toolchain usage
- Fix working directory for Yarn
- Cache and add sources
- Reuse caches
- Add additional caching improvements

### Internal

- Remove most unrelated code
- Checkpoint commits

---

## [1.0.0-alpha.1]

### Added

- Initial alpha release of Apalis Board
- Web dashboard support for Apalis task management
- Queue monitoring and task inspection UI

### Changed

- Early architecture and workflow setup

### Internal

- Initial repository setup
- CI pipeline scaffolding
