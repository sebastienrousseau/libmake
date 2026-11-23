# Changelog

All notable changes to this project are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.8] - 2026-11-22

### Changed

- The package version is 0.2.8. The v0.2.7 tag was cut while
  `Cargo.toml` still said 0.2.6, so its release workflow
  stopped at the version guard and 0.2.7 never reached crates.io.
- Dependencies: rlg 0.0.18 (#86) and noyalib 0.0.63 (#89, #91); the shared
  `rust-ci.yml` pipeline moves to pipelines 0.0.4 (#87).
- CI, documentation and release workflows default to a read-only
  `GITHUB_TOKEN`; only the jobs that push docs or create the draft
  release get `contents: write`. This closes seven CodeQL
  `actions/missing-workflow-permissions` alerts.
- Removed the unused anyhow, configparser, env_logger and vrd
  dependencies flagged by Cargo's `unused_dependencies` lint.
- `deny.toml` uses the current cargo-deny schema, so `cargo deny check`
  runs again; the policy is unchanged.
- **The MSRV is now Rust 1.89** (was 1.60), the floor shared by the
  maintainer's Rust crates. With the floor past 1.74, clippy asks for
  `io::Error::other(..)` over `io::Error::new(ErrorKind::Other, ..)`;
  the generators now use it, with the same errors.
- Generated libraries default to `--rustversion 1.89`, libmake's own
  floor, instead of 1.60.

### Fixed

- When it fails, the `libmake` binary now reports
  `Error running libmake: ...` instead of `Error running cmn: ...`, a
  message copied from another project.

### Removed

- The euxis-commons git dependency (tag v0.0.5). No libmake code used
  it, and with it go 25 crates only it pulled in (among them json5,
  ron, rust-ini, yaml-rust2 and the pest parser family). A git
  dependency also cannot be published to crates.io.
- The exported `macro_execute_and_log!` macro. Nothing in libmake used
  it, and no caller could: it expands to `$crate::log::info!`, which
  libmake does not export, so every use failed to compile (E0433).

## [Unreleased]

### Security

- ASCII art no longer depends on figlet-rs, whose every release requires
  zip 0.6 (RUSTSEC-2026-0367, RUSTSEC-2026-0368 and RUSTSEC-2026-0369,
  fixed only in zip 9): `generators::figlet` renders the bundled FIGlet
  `standard` font itself, with output identical to figlet-rs 1.0.0's.
  `load_standard_font()` now returns `generators::figlet::FIGlet`, which
  keeps `convert()`.
- euxis-commons moves to v0.0.5, whose dtt 0.0.11 uses pastey instead of
  paste (RUSTSEC-2024-0436, unmaintained).

### Added

- Repository baseline per the Repository Gold Standard template:
  CHANGELOG, SECURITY, SUPPORT, CODE_OF_CONDUCT, DEVELOPMENT, docs/
  root, and docs-lint configs.

Earlier releases predate this changelog; see the
[releases page](https://github.com/sebastienrousseau/libmake/releases)
and git history.
