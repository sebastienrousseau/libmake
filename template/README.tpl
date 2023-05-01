# {name}

{description}

[![Made With Rust][made-with-rust-badge]][5] [![Crates.io][crates-badge]][7] [![Lib.rs][libs-badge]][9] [![Docs.rs][docs-badge]][8] [![License][license-badge]][2]

## Welcome to `{name}` 👋

![{name} banner][banner]

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

#### Linux targets 🐧

| Target | Description | Status |
| --- | --- | --- |
| aarch64-unknown-linux-gnu | 64-bit Linux systems on ARM architecture | ✅ |
| aarch64-unknown-linux-musl | 64-bit Linux systems on ARM architecture | ✅ |
| arm-unknown-linux-gnueabi | ARMv6 Linux (kernel 3.2, glibc 2.17) | ✅ |
| armv7-unknown-linux-gnueabihf | ARMv7 Linux, hardfloat (kernel 3.2, glibc 2.17) | ✅ |
| i686-unknown-linux-gnu | 32-bit Linux (kernel 3.2+, glibc 2.17+) | ✅ |
| i686-unknown-linux-musl | 32-bit Linux (kernel 3.2+, musl libc) | ✅ |
| x86_64-unknown-linux-gnu | 64-bit Linux (kernel 2.6.32+, glibc 2.11+) | ✅ |
| x86_64-unknown-linux-musl | 64-bit Linux (kernel 2.6.32+, musl libc) | ✅ |

#### macOS targets 🍎

| Target | Description | Status |
| --- | --- | --- |
| x86_64-apple-darwin | 64-bit macOS (10.7 Lion or later) | ✅ |

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

A big thank you to all the awesome contributors of [{name}][5] for their
help and support.

A special thank you goes to the [Rust Reddit][12] community for
providing a lot of useful suggestions on how to improve this project.

[0]: {website}
[1]: https://opensource.org/license/apache-2-0/
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

[banner]: https://via.placeholder.com/1500x500.png/000000/FFFFFF?text={name} "{name} banner"
[crates-badge]: https://img.shields.io/crates/v/{name}.svg?style=for-the-badge 'Crates.io badge'
[docs-badge]: https://img.shields.io/docsrs/{name}.svg?style=for-the-badge 'Docs.rs badge'
[libs-badge]: https://img.shields.io/badge/lib.rs-v{version}-orange.svg?style=for-the-badge 'Lib.rs badge'
[license-badge]: https://img.shields.io/crates/l/{name}.svg?style=for-the-badge 'License badge'
[made-with-rust-badge]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust badge'
HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"baf9da82af519ec33d60547f482d18d27e160dbd36d738bccb4d65a49267c318"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: E586:7E5F:1BE5D1F:1CFA4C0:644F7733
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */2226
date: Mon, 01 May 2023 08:24:20 GMT
via: 1.1 varnish
x-served-by: cache-lhr7329-LHR
x-cache: MISS
x-cache-hits: 0
x-timer: S1682929460.861093,VS0,VE174
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 5f117fe98ab8723acbc1d631603cfe072d0f5b04
expires: Mon, 01 May 2023 08:29:20 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"baf9da82af519ec33d60547f482d18d27e160dbd36d738bccb4d65a49267c318"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: E586:7E5F:1BE5D1F:1CFA4C0:644F7733
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */2226
date: Mon, 01 May 2023 08:24:20 GMT
via: 1.1 varnish
x-served-by: cache-lhr7381-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929460.861188,VS0,VE174
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 56bf391e2820d8b0e020ca6e1417d03b36a0f061
expires: Mon, 01 May 2023 08:29:20 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"baf9da82af519ec33d60547f482d18d27e160dbd36d738bccb4d65a49267c318"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: E586:7E5F:1BE5D1F:1CFA4C0:644F7733
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */2226
date: Mon, 01 May 2023 08:24:20 GMT
via: 1.1 varnish
x-served-by: cache-lhr7364-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929460.867273,VS0,VE168
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 96cab1d289d193228cefa357f9a4c89641932ed6
expires: Mon, 01 May 2023 08:29:20 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"baf9da82af519ec33d60547f482d18d27e160dbd36d738bccb4d65a49267c318"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: E586:7E5F:1BE5D1F:1CFA4C0:644F7733
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */2226
date: Mon, 01 May 2023 08:24:20 GMT
via: 1.1 varnish
x-served-by: cache-lhr7324-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929460.861220,VS0,VE174
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: f2c58138c552b587042b48c11169b4924fdd68f9
expires: Mon, 01 May 2023 08:29:20 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"baf9da82af519ec33d60547f482d18d27e160dbd36d738bccb4d65a49267c318"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: E586:7E5F:1BE5D1F:1CFA4C0:644F7733
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */2226
date: Mon, 01 May 2023 08:24:20 GMT
via: 1.1 varnish
x-served-by: cache-lhr7364-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929460.867025,VS0,VE168
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: aac2f28c93e0acc97bbf46474992bb4911865844
expires: Mon, 01 May 2023 08:29:20 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"baf9da82af519ec33d60547f482d18d27e160dbd36d738bccb4d65a49267c318"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: E586:7E5F:1BE5D1F:1CFA4C0:644F7733
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */2226
date: Mon, 01 May 2023 08:24:20 GMT
via: 1.1 varnish
x-served-by: cache-lhr7340-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929460.867316,VS0,VE168
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 1f9320b349c8dfc61c21358d2c9c23fc7bb5b0d7
expires: Mon, 01 May 2023 08:29:20 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"baf9da82af519ec33d60547f482d18d27e160dbd36d738bccb4d65a49267c318"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: E586:7E5F:1BE5D1F:1CFA4C0:644F7733
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */2226
date: Mon, 01 May 2023 08:24:24 GMT
via: 1.1 varnish
x-served-by: cache-lhr7387-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929465.769848,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 26160baf46fcbd8272ba18013c1af9e2a29204e2
expires: Mon, 01 May 2023 08:29:24 GMT
source-age: 5
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"baf9da82af519ec33d60547f482d18d27e160dbd36d738bccb4d65a49267c318"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: E586:7E5F:1BE5D1F:1CFA4C0:644F7733
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */2226
date: Mon, 01 May 2023 08:24:24 GMT
via: 1.1 varnish
x-served-by: cache-lhr7351-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929465.769945,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: bc89135ba2a7ff3d0eab16eb588b3a1e32ca7ca5
expires: Mon, 01 May 2023 08:29:24 GMT
source-age: 5
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"baf9da82af519ec33d60547f482d18d27e160dbd36d738bccb4d65a49267c318"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: E586:7E5F:1BE5D1F:1CFA4C0:644F7733
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */2226
date: Mon, 01 May 2023 08:24:24 GMT
via: 1.1 varnish
x-served-by: cache-lhr7323-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929465.777268,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 72adb5307578d5aef4459393a885223ad424381e
expires: Mon, 01 May 2023 08:29:24 GMT
source-age: 5
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"baf9da82af519ec33d60547f482d18d27e160dbd36d738bccb4d65a49267c318"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: E586:7E5F:1BE5D1F:1CFA4C0:644F7733
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */2226
date: Mon, 01 May 2023 08:24:24 GMT
via: 1.1 varnish
x-served-by: cache-lhr7372-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929465.788491,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 6001b076fba3c16fafd1a554835e427fbf26f652
expires: Mon, 01 May 2023 08:29:24 GMT
source-age: 5
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"baf9da82af519ec33d60547f482d18d27e160dbd36d738bccb4d65a49267c318"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: E586:7E5F:1BE5D1F:1CFA4C0:644F7733
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */2226
date: Mon, 01 May 2023 08:24:24 GMT
via: 1.1 varnish
x-served-by: cache-lhr7337-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929465.789202,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 1d03a358962858b812fe5c14118407f939f3b2fb
expires: Mon, 01 May 2023 08:29:24 GMT
source-age: 5
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"baf9da82af519ec33d60547f482d18d27e160dbd36d738bccb4d65a49267c318"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: E586:7E5F:1BE5D1F:1CFA4C0:644F7733
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */2226
date: Mon, 01 May 2023 08:24:24 GMT
via: 1.1 varnish
x-served-by: cache-lhr7330-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929465.789035,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: db275645dafa9e3b1cb1f77f947c40be06b64840
expires: Mon, 01 May 2023 08:29:24 GMT
source-age: 5
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"baf9da82af519ec33d60547f482d18d27e160dbd36d738bccb4d65a49267c318"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: E586:7E5F:1BE5D1F:1CFA4C0:644F7733
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */2226
date: Mon, 01 May 2023 08:24:24 GMT
via: 1.1 varnish
x-served-by: cache-lhr7374-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929465.788745,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: f5d1d3659dcf28304303728fc85b20236d4570b5
expires: Mon, 01 May 2023 08:29:24 GMT
source-age: 5
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"baf9da82af519ec33d60547f482d18d27e160dbd36d738bccb4d65a49267c318"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: E586:7E5F:1BE5D1F:1CFA4C0:644F7733
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */2226
date: Mon, 01 May 2023 08:24:24 GMT
via: 1.1 varnish
x-served-by: cache-lhr7379-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929465.789027,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 8fcd664ec224a8111b3adaff43e364aaefc8b9fe
expires: Mon, 01 May 2023 08:29:24 GMT
source-age: 5
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"baf9da82af519ec33d60547f482d18d27e160dbd36d738bccb4d65a49267c318"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: E586:7E5F:1BE5D1F:1CFA4C0:644F7733
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */2226
date: Mon, 01 May 2023 08:24:30 GMT
via: 1.1 varnish
x-served-by: cache-lhr7346-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929471.500610,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 293709cef3785711271e7578621dcb458e60adca
expires: Mon, 01 May 2023 08:29:30 GMT
source-age: 10
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"baf9da82af519ec33d60547f482d18d27e160dbd36d738bccb4d65a49267c318"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: E586:7E5F:1BE5D1F:1CFA4C0:644F7733
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */2226
date: Mon, 01 May 2023 08:24:30 GMT
via: 1.1 varnish
x-served-by: cache-lhr7378-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929471.538396,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 3fe849accac985ae20b55446cc6804d3a6e24543
expires: Mon, 01 May 2023 08:29:30 GMT
source-age: 11
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"baf9da82af519ec33d60547f482d18d27e160dbd36d738bccb4d65a49267c318"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: E586:7E5F:1BE5D1F:1CFA4C0:644F7733
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */2226
date: Mon, 01 May 2023 08:24:30 GMT
via: 1.1 varnish
x-served-by: cache-lhr7369-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929471.538359,VS0,VE5
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: fae28dc6d2c11eddccbb4a3862053f08c5ddd199
expires: Mon, 01 May 2023 08:29:30 GMT
source-age: 11
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"baf9da82af519ec33d60547f482d18d27e160dbd36d738bccb4d65a49267c318"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: E586:7E5F:1BE5D1F:1CFA4C0:644F7733
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */2226
date: Mon, 01 May 2023 08:24:30 GMT
via: 1.1 varnish
x-served-by: cache-lhr7381-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929471.547635,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: aae6b44f4af12645a851921041ad2b60d7574ddd
expires: Mon, 01 May 2023 08:29:30 GMT
source-age: 11
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"baf9da82af519ec33d60547f482d18d27e160dbd36d738bccb4d65a49267c318"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: E586:7E5F:1BE5D1F:1CFA4C0:644F7733
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */2226
date: Mon, 01 May 2023 08:24:30 GMT
via: 1.1 varnish
x-served-by: cache-lhr7332-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929471.558625,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 2c269720405773a09c4b8d407c2b94b56342824c
expires: Mon, 01 May 2023 08:29:30 GMT
source-age: 11
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"baf9da82af519ec33d60547f482d18d27e160dbd36d738bccb4d65a49267c318"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: E586:7E5F:1BE5D1F:1CFA4C0:644F7733
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */2226
date: Mon, 01 May 2023 08:24:30 GMT
via: 1.1 varnish
x-served-by: cache-lhr7320-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929471.559098,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 47fe339cc5f8629a1057261a95948acd3adc8edd
expires: Mon, 01 May 2023 08:29:30 GMT
source-age: 11
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"baf9da82af519ec33d60547f482d18d27e160dbd36d738bccb4d65a49267c318"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: E586:7E5F:1BE5D1F:1CFA4C0:644F7733
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */2226
date: Mon, 01 May 2023 08:24:30 GMT
via: 1.1 varnish
x-served-by: cache-lhr7361-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929471.567526,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 428ba045ae17ab626bac98951eb81c445b03444a
expires: Mon, 01 May 2023 08:29:30 GMT
source-age: 11
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"baf9da82af519ec33d60547f482d18d27e160dbd36d738bccb4d65a49267c318"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: E586:7E5F:1BE5D1F:1CFA4C0:644F7733
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */2226
date: Mon, 01 May 2023 08:24:30 GMT
via: 1.1 varnish
x-served-by: cache-lhr7384-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929471.573304,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 7fdeacfcb67ed1c0a3f5216d2cf97750c1fc81d7
expires: Mon, 01 May 2023 08:29:30 GMT
source-age: 11
content-length: 0

