# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog and this project adheres to Semantic Versioning:

- https://keepachangelog.com/en/1.0.0/
- https://semver.org/

## [Unreleased]

### Added

- Initial workspace layout and placeholder crates/web dashboard.
- CI workflows for build/lint/test.
- Formatting and linting configs for Rust (`rustfmt.toml`, `clippy.toml`) and frontend (`oxlint.json`, `prettier.config.js`).
- Root `Cargo.toml` workspace and placeholder crate
- `web/relaynaut-dashboard` Vue 3 + Vite + TypeScript + Pinia scaffold (Phase 1 placeholder UI).

### Changed

- N/A

### Fixed

- N/A

## [0.1.0] - 2025-10-20

### Added

- Initial repo setup (Phase 1): repository scaffolded with `crates/`, `web/`, `ops/`, `docs/`, `ci/`, and `ai/` folders.
- Project builds and structure ready; placeholder Rust crates compile under the workspace and the Vue dashboard scaffold is present.
- Linting and formatting tooling added; CI configured to run build/lint/test before merges.

---

For release process and changelog contribution guidance, see `docs/semver-notes.md` (to be added).
