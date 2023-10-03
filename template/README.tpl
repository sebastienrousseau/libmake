<!-- markdownlint-disable MD033 MD041 -->

<img
src="https://via.placeholder.com/199x199.png/E82440/78C9FF?text={name}"
alt="{name}'s logo"
height="199"
width="199"
align="right"
/>

<!-- markdownlint-enable MD033 MD041 -->

# {name}

{description}

<!-- markdownlint-disable MD033 MD041 -->
<center>
<!-- markdownlint-enable MD033 MD041 -->

[![Made With Rust][made-with-rust-badge]][5]
[![Crates.io][crates-badge]][7]
[![Lib.rs][libs-badge]][9]
[![Docs.rs][docs-badge]][8]
[![License][license-badge]][2]

• [Website][0]
• [Documentation][8]
• [Report Bug][3]
• [Request Feature][3]
• [Contributing Guidelines][4]

<!-- markdownlint-disable MD033 MD041 -->
</center>
<!-- markdownlint-enable MD033 MD041 -->

![divider][divider]

## Overview 📖

{description}

## Features ✨

- Feature 1
- Feature 2
- Feature 3

## Getting Started 🚀

It takes just a few minutes to get up and running with `{name}`.

### Installation

To install `{name}`, you need to have the Rust toolchain installed on
your machine. You can install the Rust toolchain by following the
instructions on the [Rust website][13].

Once you have the Rust toolchain installed, you can install `{name}`
using the following command:

```shell
cargo install {name}
```

You can then run the help command to see the available options:

```shell
{name} --help
```

### Requirements

The minimum supported Rust toolchain version is currently Rust
**{rustversion}** or later (stable).

### Platform support

`{name}` is supported and tested on the following platforms:

### Tier 1 platforms 🏆

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

### Tier 2 platforms 🥈

| | Operating System | Target | Description |
| --- | --- | --- | --- |
| ✅ | Linux   | aarch64-unknown-linux-musl | 64-bit Linux systems on ARM architecture |
| ✅ | Linux   | arm-unknown-linux-gnueabi | ARMv6 Linux (kernel 3.2, glibc 2.17) |
| ✅ | Linux   | arm-unknown-linux-gnueabihf | ARMv7 Linux, hardfloat (kernel 3.2, glibc 2.17) |
| ✅ | Linux   | armv7-unknown-linux-gnueabihf | ARMv7 Linux, hardfloat (kernel 3.2, glibc 2.17) |
| ✅ | Linux   | mips-unknown-linux-gnu | MIPS Linux (kernel 2.6.32+, glibc 2.11+) |
| ✅ | Linux   | mips64-unknown-linux-gnuabi64 | MIPS64 Linux (kernel 2.6.32+, glibc 2.11+) |
| ✅ | Linux   | mips64el-unknown-linux-gnuabi64 | MIPS64 Linux (kernel 2.6.32+, glibc 2.11+) |
| ✅ | Linux   | mipsel-unknown-linux-gnu | MIPS Linux (kernel 2.6.32+, glibc 2.11+) |
| ✅ | macOS   | aarch64-apple-darwin | 64-bit macOS (10.7 Lion or later) |
| ✅ | Windows | aarch64-pc-windows-msvc | 64-bit Windows (7 or later) |

The [GitHub Actions][10] shows the platforms in which the `{name}`
library tests are run.

### Documentation

> ℹ️ **Info:** Please check out our [website][0] for more information.
You can find our documentation on [docs.rs][8], [lib.rs][9] and
[crates.io][7].

## Usage 📖

To use the `{name}` library in your project, add the following to your
`Cargo.toml` file:

```toml
[dependencies]
{name} = "{version}"
```

Add the following to your `main.rs` file:

```rust
extern crate {name};
use {name}::*;
```

then you can use the functions in your application code.

### Examples

To get started with `{name}`, you can use the examples provided in the
`examples` directory of the project.

To run the examples, clone the repository and run the following command
in your terminal from the project root directory.

```shell
cargo run --example {name}
```

## Semantic Versioning Policy 🚥

For transparency into our release cycle and in striving to maintain
backward compatibility, `{name}` follows [semantic versioning][6].

## License 📝

The project is licensed under the terms of {license}.

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

A big thank you to all the awesome contributors of [{name}][5] for their
help and support.

A special thank you goes to the [Rust Reddit][12] community for
providing a lot of useful suggestions on how to improve this project.

[0]: {website}
[2]: http://opensource.org/licenses/MIT
[3]: {repository}/{name}/issues
[4]: {repository}/{name}/blob/main/CONTRIBUTING.md
[5]: {repository}/{name}/graphs/contributors
[6]: http://semver.org/
[7]: https://crates.io/crates/{name}
[8]: https://docs.rs/{name}
[9]: https://lib.rs/crates/{name}
[10]: {repository}/{name}/actions
[11]: https://www.rust-lang.org/policies/code-of-conduct
[12]: https://www.reddit.com/r/rust/
[13]: https://www.rust-lang.org/learn/get-started

[crates-badge]: https://img.shields.io/crates/v/{name}.svg?style=for-the-badge 'Crates.io badge'
[divider]: https://kura.pro/common/images/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/{name}.svg?style=for-the-badge 'Docs.rs badge'
[libs-badge]: https://img.shields.io/badge/lib.rs-v{version}-orange.svg?style=for-the-badge 'Lib.rs badge'
[license-badge]: https://img.shields.io/crates/l/{name}.svg?style=for-the-badge 'License badge'
[made-with-rust-badge]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust badge'