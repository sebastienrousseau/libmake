# Changelog

All notable changes to this project are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.8] - Unreleased

### Changed

- **The MSRV is now Rust 1.89** (was 1.60), the floor shared by the
  maintainer's Rust crates. With the floor past 1.74, clippy asks for
  `io::Error::other(..)` over `io::Error::new(ErrorKind::Other, ..)`;
  the generators now use it, with the same errors.

## [Unreleased]

### Added

- Repository baseline per the Repository Gold Standard template:
  CHANGELOG, SECURITY, SUPPORT, CODE_OF_CONDUCT, DEVELOPMENT, docs/
  root, and docs-lint configs.

Earlier releases predate this changelog; see the
[releases page](https://github.com/sebastienrousseau/libmake/releases)
and git history.
