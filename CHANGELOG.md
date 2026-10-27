# Changelog

All notable changes to this project are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
