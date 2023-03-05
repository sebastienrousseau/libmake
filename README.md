# LibMake

## A code generator to reduce repetitive tasks and build high-quality Rust libraries

[![Made With Rust][made-with-rust-badge]][5] [![Crates.io][crates-badge]][7] [![Lib.rs][libs-badge]][9] [![Docs.rs][docs-badge]][8] [![License][license-badge]][2] [![Codecov][codecov-badge]][14]

## Welcome to `libmake` 👋

![libmake Banner][banner]

<!-- markdownlint-disable MD033 -->
<center>

**[Website][0] • [Documentation][8] • [Report Bug][3] • [Request Feature][3] • [Contributing Guidelines][4]**

</center>

<!-- markdownlint-enable MD033 -->

## Overview 📖

`LibMake` is a tool designed to quickly help creating high-quality Rust
libraries by generating a set of pre-filled and pre-defined templated
files. This opinionated boilerplate scaffolding tool aims to greatly
reduces development time and minimizes repetitive tasks, allowing you to
focus on your business logic while enforcing standards, best practices,
consistency, and providing style guides for your library.

With `LibMake`, you can easily generate a new Rust library code base
structure with all the necessary files, layouts, build configurations,
code, tests, benchmarks, documentation, and much more in a matter of
seconds.

The library is designed to be used as a command-line tool. It is
available on [Crates.io][7] and [Lib.rs][8].

## Features ✨

`LibMake` offers the following features and benefits:

- Create your Rust library with ease using the command line interface or
  by providing a configuration file in CSV, JSON, or YAML format.
- Rapidly generate new library projects with a pre-defined structure and
  boilerplate code that you can customize with your own template.
- Automatically generate basic functions, methods, and macros to get you
  started with your Rust library.
- Enforce best practices and standards with starter documentation, test
  suites, and benchmark suites that are designed to help you get up and
  running quickly.

## Getting Started 🚀

It takes just a few minutes to get up and running with `LibMake`.

### Installation

To install `LibMake`, you need to have the Rust toolchain installed on
your machine. You can install the Rust toolchain by following the
instructions on the [Rust website][13].

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

The minimum supported Rust toolchain version is currently Rust `1.67.1`
or later (stable).

### Platform support

`LibMake` is supported and tested on the following platforms:

#### FreeBSD targets 🐬

| Target | Description | Status |
|--------|-------------|--------|
| x86_64-unknown-freebsd | 64-bit FreeBSD on x86-64 | ✅ Tested |

#### Linux targets 🐧

| Target | Description | Status |
|--------|-------------|--------|
| aarch64-unknown-linux-gnu | 64-bit Linux systems on ARM architecture | ✅ Tested |
| aarch64-unknown-linux-musl | 64-bit Linux systems on ARM architecture | ✅ Tested |
| arm-unknown-linux-gnueabi | ARMv6 Linux (kernel 3.2, glibc 2.17) | ✅ Tested |
| armv7-unknown-linux-gnueabihf | ARMv7 Linux, hardfloat (kernel 3.2, glibc 2.17) | ✅ Tested |
| i686-unknown-linux-gnu | 32-bit Linux (kernel 3.2+, glibc 2.17+) | ✅ Tested |
| i686-unknown-linux-musl | 32-bit Linux (kernel 3.2+, musl libc) | ✅ Tested |
| x86_64-unknown-linux-gnu | 64-bit Linux (kernel 2.6.32+, glibc 2.11+) | ✅ Tested |
| x86_64-unknown-linux-musl | 64-bit Linux (kernel 2.6.32+, musl libc) | ✅ Tested |

#### Illumos targets 🌞

| Target | Description | Status |
|--------|-------------|--------|
| x86_64-unknown-illumos | 64-bit Illumos on x86-64 | ✅ Tested |

#### macOS targets 🍎

| Target | Description | Status |
|--------|-------------|--------|
| aarch64-apple-darwin | 64-bit macOS on Apple Silicon | ✅ Tested |
| x86_64-apple-darwin | 64-bit macOS (10.7 Lion or later) | ✅ Tested |

The [GitHub Actions][10] shows the platforms in which the `LibMake`
library tests are run.

### Documentation

> ℹ️ **Info:** Please check out our [website][0] for more information.
You can find our documentation on [docs.rs][8], [lib.rs][9] and
[crates.io][7].

## Usage 📖

### Command-line interface

`LibMake` provides a command-line interface to generate a new library
project.

#### Generate a new library using a CSV file

The following command generates a library template from a CSV file.

Have a look at the `tests/data/mylibrary.csv` file for an example and
feel free to use it for your own library as a template.

```shell
libmake -f tests/data/mylibrary.csv
```

#### Generate a new library using a JSON file

The following command generates a library template from a JSON file.

Have a look at the `tests/data/mylibrary.json` file for an example and
feel free to use it for your own library as a template.

```shell
libmake -f tests/data/mylibrary.json
```

#### Generate a new library using a YAML file

The following command generates a library template from a YAML file.

Have a look at the `tests/data/mylibrary.yaml` file for an example and
feel free to use it for your own library as a template.

```shell
libmake -f tests/data/mylibrary.yaml
```

#### Generate a new library using the command-line interface

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
    --rustversion "1.67.1" \
    --version "0.1.0" \
    --website "https://example.com/john-smith"
```

### Examples

To get started with `LibMake`, you can use the examples provided in the
`examples` directory of the project.

To run the examples, clone the repository and run the following command
in your terminal from the project root directory.

#### Generate a new library using a CSV file

The following code uses the `generate_from_csv` function from the
`libmake` crate to generate a library template from a CSV file.

Have a look at the `tests/data/mylibrary.csv` file for an example and
feel free to use it for your own library as a template.

The CSV file contains the following header columns:

- `author` - The author of the library (e.g. `John Doe`)
- `build` - The build configuration file name (e.g. `build.rs`)
- `categories` - The categories of the library from Crates.io (e.g. `cli,development`)
- `csv` - The CSV file path to use for the library template generation (e.g. `/tests/data/mylibrary.csv`)
- `description` - The description of the library (e.g. `My library is a great library`)
- `documentation` - The documentation of the library (e.g. `https://docs.rs/mylibrary`)
- `edition` - The edition of the library (e.g. `2021`)
- `email` - The email of the author of the library (e.g. `john.doe@gmail.com`)
- `homepage` - The homepage of the library (e.g. `https://mylibrary.com`)
- `keywords` - The keywords of the library from Crates.io (e.g. `cli,development`)
- `license` - The license of the library (e.g. `MIT OR Apache-2.0`)
- `name` - The name of the library (e.g. `mylibrary`)
- `output` - The output directory path to use for the library template generation (e.g. `/tmp/mylibrary`)
- `readme` - The README file name (e.g. `README.md`)
- `repository` - The repository of the library (e.g. `https://github.com/mylibrary/mylibrary`)
- `rustversion` - The Rust version of the library (e.g. `1.56.0`)
- `version` - The version of the library (e.g. `0.1.1`)
- `website` - The website of the author of the library (e.g. `https://johndoe.com`)

Then you need to create a row in your CSV file with the values for each

You can now run the following command to generate a new library using

```shell
cargo run --example generate_from_csv
```

#### Generate a new library using a JSON file

The following code uses the `generate_from_json` function from the
`libmake` crate to generate a library template from a JSON file.

```shell
cargo run --example generate_from_json
```

#### Generate a new library using a YAML file

The following code uses the `generate_from_yaml` function from the
`libmake` crate to generate a library template from a YAML file.

```shell
cargo run --example generate_from_yaml
```

## Semantic Versioning Policy 🚥

For transparency into our release cycle and in striving to maintain
backward compatibility, `libmake` follows [semantic versioning][6].

## License 📝

The project is licensed under the terms of both the MIT license and the
Apache License (Version 2.0).

- [Apache License, Version 2.0][1]
- [MIT license][2]

## Contribution 🤝

We welcome all people who want to contribute. Please see the
[contributing instructions][4] for more information.

Contributions in any form (issues, pull requests, etc.) to this project
must adhere to the [Rust's Code of Conduct][11].

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

## Acknowledgements 💙

A big thank you to all the awesome contributors of [libmake][5] for their
help and support. A special thank you goes to the [Rust Reddit][12]
community for providing a lot of useful suggestions on how to improve
this project.

[0]: https://libmake.com
[1]: https://opensource.org/license/apache-2-0/
[2]: http://opensource.org/licenses/MIT
[3]: https://github.com/sebastienrousseau/libmake/issues
[4]: https://github.com/sebastienrousseau/libmake/blob/main/CONTRIBUTING.md
[5]: https://github.com/sebastienrousseau/libmake/graphs/contributors
[6]: http://semver.org/
[7]: https://crates.io/crates/libmake
[8]: https://docs.rs/libmake
[9]: https://lib.rs/crates/libmake
[10]: https://github.com/sebastienrousseau/libmake/actions
[11]: https://www.rust-lang.org/policies/code-of-conduct
[12]: https://www.reddit.com/r/rust/
[13]: https://www.rust-lang.org/learn/get-started
[14]: https://codecov.io/github/sebastienrousseau/libmake?branch=main

[banner]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/libmake/logo/logo-libmake.svg "libmake Banner"
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/libmake?style=for-the-badge&token=Q9KJ6XXL67 'Codecov'
[crates-badge]: https://img.shields.io/crates/v/libmake.svg?style=for-the-badge 'Crates.io Badge'
[docs-badge]: https://img.shields.io/docsrs/libmake.svg?style=for-the-badge 'Docs.rs Badge'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.1.1-orange.svg?style=for-the-badge 'Lib.rs Badge'
[license-badge]: https://img.shields.io/crates/l/libmake.svg?style=for-the-badge 'License Badge'
[made-with-rust-badge]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust Badge'
