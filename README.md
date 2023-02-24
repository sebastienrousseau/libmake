# libmake

A Rust library generator that helps create high-quality Rust libraries quickly and easily.

[![Made With Rust][made-with-rust-badge]][5]
[![Crates.io][crates-badge]][7]
[![Lib.rs][libs-badge]][9]
[![Docs.rs][docs-badge]][8]
[![License][license-badge]][2]

## Welcome to the `libmake` Rust Library 👋

![libmake Banner][banner]

<!-- markdownlint-disable MD033 -->
<center>

**[Website][0]
• [Documentation][8]
• [Report Bug][3]
• [Request Feature][3]
• [Contributing Guidelines][4]**

</center>

<!-- markdownlint-enable MD033 -->

## Overview 📖

A Rust library generator that helps create high-quality Rust libraries quickly and easily.

## Features ✨

- Generates a new Rust library manually
- Generates a new Rust library from a CSV file.

## Installation 📦

It takes just a few minutes to get up and running with the `libmake` Rust
library.

### Requirements

The minimum supported Rust toolchain version is currently Rust
**1.66.1** or later (stable).

### Platform support

`libmake` is supported and tested on the following platforms:

- Platform 1 (e,g. Linux, Windows, macOS, etc.)

The [GitHub Actions][10] shows the platforms in which the `libmake`
library tests are run.

### Documentation

> ℹ️ **Info:** Please check out our [website][0] for more information.
You can find our documentation on [docs.rs][8], [lib.rs][9] and
[crates.io][7].

## Usage 📖

To use the `libmake` library in your project, add the following to your
`Cargo.toml` file:

```toml
[dependencies]
libmake = "0.0.1"
```

Add the following to your `main.rs` file:

```rust
extern crate libmake;
use libmake::*;
```

then you can use the functions in your application code.

### Examples

The `libmake` library comes with a set of examples that you can use to
get started.

Library examples are located in the `examples` directory of the project.
To run the examples, clone the repository and run the following command
in your terminal from the project root directory.

```shell
cargo run --example libmake
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

[0]: https://test.com
[1]: https://opensource.org/license/apache-2-0/
[2]: http://opensource.org/licenses/MIT
[3]: https://github.com/sebastienrousseau/libmake.git/libmake/issues
[4]: https://github.com/sebastienrousseau/libmake.git/libmake/blob/main/contributing.md
[5]: https://github.com/sebastienrousseau/libmake.git/libmake/graphs/contributors
[6]: http://semver.org/
[7]: https://crates.io/crates/libmake
[8]: https://docs.rs/libmake
[9]: https://lib.rs/crates/libmake
[10]: https://github.com/sebastienrousseau/libmake.git/libmake/actions
[11]: https://www.rust-lang.org/policies/code-of-conduct
[12]: https://www.reddit.com/r/rust/

[banner]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/libmake/logo/logo-libmake.svg "libmake Banner"
[crates-badge]: https://img.shields.io/crates/v/libmake.svg?style=for-the-badge 'Crates.io Badge'
[docs-badge]: https://img.shields.io/docsrs/libmake.svg?style=for-the-badge 'Docs.rs Badge'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.1-orange.svg?style=for-the-badge 'Lib.rs Badge'
[license-badge]: https://img.shields.io/crates/l/libmake.svg?style=for-the-badge 'License Badge'
[made-with-rust-badge]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust Badge'
