<!-- markdownlint-disable MD033 MD041 -->

<img src="https://kura.pro/libmake/images/logos/libmake.webp"
alt="LibMake logo" width="261" align="right" />

<!-- markdownlint-enable MD033 MD041 -->
# LibMake

A code generator to reduce repetitive tasks and build high-quality Rust libraries.

*Part of the [Mini Functions][00] family of libraries.*

<!-- markdownlint-disable MD033 MD041 -->
<center>
<!-- markdownlint-enable MD033 MD041 -->

![Banner of Libmake][banner]

[![Made With Rust][made-with-rust-badge]][13] [![Crates.io][crates-badge]][08] [![Lib.rs][libs-badge]][10] [![Docs.rs][docs-badge]][09] [![License][license-badge]][03] [![Codecov][codecov-badge]][14]

• [Website][01] • [Documentation][09] • [Report Bug][04] • [Request Feature][04] • [Contributing Guidelines][05]

<!-- markdownlint-disable MD033 MD041 -->
</center>
<!-- markdownlint-enable MD033 MD041 -->

<!-- markdownlint-enable MD033 -->

![divider][divider]

## Overview

`LibMake` is a tool designed to quickly help creating high-quality Rust libraries by generating a set of pre-filled and pre-defined templated files. This opinionated boilerplate scaffolding tool aims to greatly reduce development time and minimizes repetitive tasks, allowing you to focus on your business logic while enforcing standards, best practices, consistency, and providing style guides for your library.

With `LibMake`, you can easily generate a new Rust library code base structure with all the necessary files, layouts, build configurations, code, tests, benchmarks, documentation, and much more in a matter of seconds.

The library is designed to be used as a command-line tool. It is available on [Crates.io][07] and [Lib.rs][08].

## Table of Contents

- [LibMake](#libmake)
  - [Overview](#overview)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Installation](#installation)
    - [Requirements](#requirements)
      - [Tier 1 platforms](#tier-1-platforms)
      - [Tier 2 platforms](#tier-2-platforms)
    - [Documentation](#documentation)
  - [Usage](#usage)
    - [Command-line interface](#command-line-interface)
      - [Generate a new library using a CSV file](#generate-a-new-library-using-a-csv-file)
      - [Generate a new library using a JSON file](#generate-a-new-library-using-a-json-file)
      - [Generate a new library using a TOML file](#generate-a-new-library-using-a-toml-file)
      - [Generate a new library using a YAML file](#generate-a-new-library-using-a-yaml-file)
      - [Generate a new library using the command-line interface (CLI) directly](#generate-a-new-library-using-the-command-line-interface-cli-directly)
    - [Examples](#examples)
  - [Semantic Versioning Policy](#semantic-versioning-policy)
  - [License](#license)
  - [Contribution](#contribution)
  - [Acknowledgements](#acknowledgements)

## Features

`LibMake` offers the following features and benefits:

- **Simplicity**: Create Rust libraries effortlessly via CLI or configuration files in CSV, JSON, TOML, or YAML.
- **Speed**: Instantly scaffold new libraries with a standard structure and essential boilerplate.
- **Automation**: Generate predefined GitHub Actions workflows for streamlined development and testing.
- **Foundation**: Jumpstart your project with automatically generated functions, methods, and macros.
- **Standards**: Embrace best practices from the start with starter documentation, tests, and benchmarks.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

You will need Rust and Cargo installed on your system. If you don't have them installed, you can install them from the official Rust website.

It takes just a few seconds to get up and running with `LibMake`.

### Installation

Once you have the Rust toolchain installed, you can install `LibMake`
using the following command:

```shell
cargo install libmake
```

You can then run the help command to see the available options:

```shell
libmake --help
```

### Requirements

The minimum supported Rust toolchain version is currently Rust `1.71.1` or later (stable).

`LibMake` is supported and has been tested on the following platforms:

#### Tier 1 platforms

Rust Tier 1 targets are officially supported and guaranteed to work.

| | Operating System | Target | Description |
| --- | --- | --- | --- |
| ✅ | Linux   | aarch64-unknown-linux-gnu | 64-bit Linux systems on ARM architecture |
| ✅ | Linux   | i686-unknown-linux-gnu | 32-bit Linux (kernel 3.2+, glibc 2.17+) |
| ✅ | Linux   | x86_64-unknown-linux-gnu | 64-bit Linux (kernel 2.6.32+, glibc 2.11+) |
| ✅ | macOS   | x86_64-apple-darwin | 64-bit macOS (10.7 Lion or later) |
| ✅ | Windows | i686-pc-windows-gnu | 32-bit Windows (7 or later) |
| ✅ | Windows | i686-pc-windows-msvc | 32-bit Windows (7 or later) |
| ✅ | Windows | x86_64-pc-windows-gnu | 64-bit Windows (7 or later) |
| ✅ | Windows | x86_64-pc-windows-msvc | 64-bit Windows (7 or later) |

#### Tier 2 platforms

Rust Tier 2 targets are supported for building, but not necessarily running.

| | Operating System | Target | Description |
| --- | --- | --- | --- |
| ✅ | Linux   | aarch64-unknown-linux-musl | 64-bit Linux systems on ARM architecture |
| ✅ | Linux   | arm-unknown-linux-gnueabi | ARMv6 Linux (kernel 3.2, glibc 2.17) |
| ✅ | Linux   | arm-unknown-linux-gnueabihf | ARMv7 Linux, hardfloat (kernel 3.2, glibc 2.17) |
| ✅ | Linux   | armv7-unknown-linux-gnueabihf | ARMv7 Linux, hardfloat (kernel 3.2, glibc 2.17) |
| ✅ | macOS   | aarch64-apple-darwin | 64-bit macOS (10.7 Lion or later) |
| ✅ | Windows | aarch64-pc-windows-msvc | 64-bit Windows (7 or later) |

The [GitHub Actions][11] shows the platforms in which the `LibMake`
library tests are run.

Should you encounter any issues with the library on any of the above
platforms, please [report a bug][04]. We will do our best to resolve the
issue as soon as possible. If you would like to contribute to help us to
support additional platforms, please submit a pull request.

### Documentation

**Info:** Do check out our [website][01] for more information. You can find our documentation on [docs.rs][09], [lib.rs][10] and [crates.io][08].

## Usage

### Command-line interface

`LibMake` provides a command-line interface to generate a new library project. There are a few options available to help you get started.

#### Generate a new library using a CSV file

The following command generates a library template from a CSV file.

Have a look at the `tests/data/mylibrary.csv` file for an example and
feel free to use it for your own library as a template.

```shell
libmake --csv tests/data/mylibrary.csv
```

or locally if you have cloned the repository:

```shell
cargo run -- --csv tests/data/mylibrary.csv
```

#### Generate a new library using a JSON file

The following command generates a library template from a JSON file.

Have a look at the `tests/data/mylibrary.json` file for an example and
feel free to use it for your own library as a template.

```shell
libmake --json tests/data/mylibrary.json
```

or locally if you have cloned the repository:

```shell
cargo run -- --json tests/data/mylibrary.json
```

#### Generate a new library using a TOML file

The following command generates a library template from a TOML file.

Have a look at the `tests/data/mylibrary.toml` file for an example and
feel free to use it for your own library as a template.

```shell
libmake --toml tests/data/mylibrary.toml
```

or locally if you have cloned the repository:

```shell
cargo run -- --toml tests/data/mylibrary.toml
```

#### Generate a new library using a YAML file

The following command generates a library template from a YAML file.

Have a look at the `tests/data/mylibrary.yaml` file for an example and
feel free to use it for your own library as a template.

```shell
libmake --yml tests/data/mylibrary.yaml
```

or locally if you have cloned the repository:

```shell
cargo run -- --yml tests/data/mylibrary.yaml
```

#### Generate a new library using the command-line interface (CLI) directly

The following command generates a library template using the command-line
interface.

```shell
libmake \
    --author "John Smith" \
    --build "build.rs" \
    --categories "['category 1', 'category 2', 'category 3']" \
    --description "A Rust library for doing cool things" \
    --documentation "https://docs.rs/my_library" \
    --edition "2021" \
    --email "john.smith@example.com" \
    --homepage "https://my_library.rs" \
    --keywords "['rust', 'library', 'cool']" \
    --license "MIT" \
    --name "my_library" \
    --output "my_library" \
    --readme "README.md" \
    --repository "https://github.com/example/my_library" \
    --rustversion "1.71.1" \
    --version "0.1.0" \
    --website "https://example.com/john-smith"
```

or locally if you have cloned the repository:

```shell
cargo run -- --author "John Smith" \
    --build "build.rs" \
    --categories "['category 1', 'category 2', 'category 3']" \
    --description "A Rust library for doing cool things" \
    --documentation "https://docs.rs/my_library" \
    --edition "2021" \
    --email "john.smith@example.com" \
    --homepage "https://my_library.rs" \
    --keywords "['rust', 'library', 'cool']" \
    --license "MIT" \
    --name "my_library" \
    --output "my_library" \
    --readme "README.md" \
    --repository "https://github.com/example/my_library" \
    --rustversion "1.71.1" \
    --version "0.1.0" \
    --website "https://example.com/john-smith"
```

### Examples

To get started with `LibMake`, you can use the examples provided in the `examples` directory of the project.

To run the examples, clone the repository and run the following command in your terminal from the project root directory.

| Example | Description | Command |
|---------|-------------|---------|
| `generate_from_args` | Generates a library template using the command-line interface. | `cargo run --example generate_from_args` |
| `generate_from_config` | Generates a library template from a configuration file. | `cargo run --example generate_from_config` |
| `generate_from_csv` | Generates a library template from a CSV file. | `cargo run --example generate_from_csv` |
| `generate_from_json` | Generates a library template from a JSON file. | `cargo run --example generate_from_json` |
| `generate_from_toml` | Generates a library template from a TOML file. | `cargo run --example generate_from_toml` |
| `generate_from_yaml` | Generates a library template from a YAML file. | `cargo run --example generate_from_yaml` |

## Semantic Versioning Policy

For transparency into our release cycle and in striving to maintain
backward compatibility, `libmake` follows [semantic versioning][07].

## License

The project is licensed under the terms of both the MIT license and the
Apache License (Version 2.0).

- [Apache License, Version 2.0][02]
- [MIT license][03]

## Contribution

We welcome all people who want to contribute. Please see the
[contributing instructions][05] for more information.

Contributions in any form (issues, pull requests, etc.) to this project
must adhere to the [Rust's Code of Conduct][12].

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

## Acknowledgements

A big thank you to all the awesome contributors of [libmake][06] for their help
and support. A special thank you goes to the [Rust Reddit][15] community for
providing a lot of useful suggestions on how to improve this project.

[00]: https://minifunctions.com/libmake "Mini Functions"
[01]: https://libmake.com "LibMake"
[02]: https://opensource.org/license/apache-2-0/ "Apache License, Version 2.0"
[03]: http://opensource.org/licenses/MIT "MIT license"
[04]: https://github.com/sebastienrousseau/libmake/issues "Issues"
[05]: https://github.com/sebastienrousseau/libmake/blob/main/CONTRIBUTING.md "Contributing"
[06]: https://github.com/sebastienrousseau/libmake/graphs/contributors "Contributors"
[07]: http://semver.org/ "Semantic Versioning"
[08]: https://crates.io/crates/libmake "LibMake on crates.io"
[09]: https://docs.rs/libmake "LibMake on docs.rs"
[10]: https://lib.rs/crates/libmake "LibMake on lib.rs"
[11]: https://github.com/sebastienrousseau/libmake/actions "GitHub Actions"
[12]: https://www.rust-lang.org/policies/code-of-conduct "Rust's Code of Conduct"
[13]: https://www.rust-lang.org "The Rust Programming Language"
[14]: https://codecov.io/gh/sebastienrousseau/libmake "Codecov"
[15]: https://www.reddit.com/r/rust/ "Rust Reddit"

[banner]: https://kura.pro/libmake/images/titles/title-libmake.svg "LibMake Banner"
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/libmake?style=for-the-badge&token=Q9KJ6XXL67 "Codecov Badge"
[crates-badge]: https://img.shields.io/crates/v/libmake.svg?style=for-the-badge "Crates.io Badge"
[divider]: https://kura.pro/common/images/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/libmake.svg?style=for-the-badge "Docs.rs Badge"
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.2.2-orange.svg?style=for-the-badge "Lib.rs Badge"
[license-badge]: https://img.shields.io/crates/l/libmake.svg?style=for-the-badge "License Badge"
[made-with-rust-badge]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust "Made With Rust Badge"
