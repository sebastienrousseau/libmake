// Copyright © 2023 {name}. All rights reserved.
// SPDX-License-Identifier: {license}
//!
//! # {name} 🦀
//!
//! [![{name}](https://via.placeholder.com/1500x500.png/000000/FFFFFF?text={name})]({website} "{name} - {description}")
//!
//! {description}
//!
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org "Rust")
//! [![Crates.io](https://img.shields.io/crates/v/{name}.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/{name} "Crates.io")
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v{version}-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/{name} "Lib.rs")
//! [![License](https://img.shields.io/crates/l/{name}.svg?style=for-the-badge&color=007EC6&labelColor=03589B)]({license}  "MIT or Apache License, Version 2.0")
//!
//! ## Overview
//!
//! {description}
//!
//! ## Features
//!
//! - Serialization and deserialization of data structures to JSON format
//! - ...
//! - ...
//!
//! ## Usage
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! {name} = "{version}"
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! ```
//!
//! ## Examples and Usage
//!
//! Check out the examples folder for helpful snippets of code that
//! demonstrate how to use the `{name}` library. You can also check out
//! the [documentation](https://docs.rs/{name}) for more information on
//! how to use the library.
//!
//! ## License
//!
//! The project is licensed under the terms of both the MIT license and
//! the Apache License (Version 2.0).
//!
//! - [Apache License, Version 2.0](LICENSE-APACHE.md)
//! - [MIT license](LICENSE-MIT.md)
//!
//! - [Apache License, Version 2.0](https://opensource.org/license/apache-2-0/ "Apache License, Version 2.0")
//! - [MIT license](http://opensource.org/licenses/MIT "MIT license")
//!
#![forbid(unsafe_code)]
#![warn(unreachable_pub)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![doc(
    html_favicon_url = "",
    html_logo_url = "",
    html_root_url = "https://docs.rs/{name}"
)]
#![crate_name = "{name}"]
#![crate_type = "lib"]

use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone)]
#[allow(non_camel_case_types)]
/// {name} is a data structure that ...
pub struct {name} {
    // Add any data fields needed here
}

/// This is the main entry point for the my_library library.
pub fn run() -> Result<(), Box<dyn Error>> {
    // Add your code here
    let name = "my_library";
    println!("Hello, {}!", { name }.to_uppercase());
    Ok(())
}


impl {name} {
    /// Creates a new instance of {name}
    pub fn new() -> Self {
        Self {
            // Initialize any data fields here
        }
    }
}

impl Default for {name} {
    /// Creates a new instance of {name} with default values
    fn default() -> Self {
        Self::new()
    }
}


HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"638e5f014e10760fc1deaf86440990400c1ffdeb529f046139de9a428be6e869"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: FFC8:286F:1B848EA:1C990A0:644F7731
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */1336
date: Mon, 01 May 2023 08:24:19 GMT
via: 1.1 varnish
x-served-by: cache-lhr7386-LHR
x-cache: MISS
x-cache-hits: 0
x-timer: S1682929459.869263,VS0,VE170
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: cc11ea1ded5f26284c87d0b69b03d2d774837105
expires: Mon, 01 May 2023 08:29:19 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"638e5f014e10760fc1deaf86440990400c1ffdeb529f046139de9a428be6e869"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: FFC8:286F:1B848EA:1C990A0:644F7731
content-encoding: gzip
accept-ranges: bytes
date: Mon, 01 May 2023 08:24:19 GMT
via: 1.1 varnish
x-served-by: cache-lhr7320-LHR
x-cache: HIT
x-cache-hits: 1
x-timer: S1682929459.876064,VS0,VE163
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 2c2e8ec1167d7d7c608a2b610883885a8933401a
expires: Mon, 01 May 2023 08:29:19 GMT
source-age: 0
content-range: bytes */1336
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"638e5f014e10760fc1deaf86440990400c1ffdeb529f046139de9a428be6e869"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: D87C:0A43:109DCC0:1147932:644F7732
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */1336
date: Mon, 01 May 2023 08:24:19 GMT
via: 1.1 varnish
x-served-by: cache-lhr7359-LHR
x-cache: MISS
x-cache-hits: 0
x-timer: S1682929459.869305,VS0,VE180
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 56eac7059ccc784051e85887eac96ad09de6102c
expires: Mon, 01 May 2023 08:29:19 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"638e5f014e10760fc1deaf86440990400c1ffdeb529f046139de9a428be6e869"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: D87C:0A43:109DCC0:1147932:644F7732
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */1336
date: Mon, 01 May 2023 08:24:19 GMT
via: 1.1 varnish
x-served-by: cache-lhr7355-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929459.879233,VS0,VE170
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 58f7f11ae3ad0ec51e6ad4c77bb93210fbab26fe
expires: Mon, 01 May 2023 08:29:19 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"638e5f014e10760fc1deaf86440990400c1ffdeb529f046139de9a428be6e869"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: D87C:0A43:109DCC0:1147932:644F7732
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */1336
date: Mon, 01 May 2023 08:24:19 GMT
via: 1.1 varnish
x-served-by: cache-lhr7383-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929459.876083,VS0,VE173
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: deb458112a2480adeb8afe45c4596152dc209911
expires: Mon, 01 May 2023 08:29:19 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"638e5f014e10760fc1deaf86440990400c1ffdeb529f046139de9a428be6e869"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: D87C:0A43:109DCC0:1147932:644F7732
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */1336
date: Mon, 01 May 2023 08:24:19 GMT
via: 1.1 varnish
x-served-by: cache-lhr7351-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929459.869477,VS0,VE180
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 76aa55ec48efe2d1401d55e555a7fa376a5eedde
expires: Mon, 01 May 2023 08:29:19 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"638e5f014e10760fc1deaf86440990400c1ffdeb529f046139de9a428be6e869"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: D87C:0A43:109DCC0:1147932:644F7732
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */1336
date: Mon, 01 May 2023 08:24:24 GMT
via: 1.1 varnish
x-served-by: cache-lhr7342-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929464.162339,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 0f271fcbcdb0c66db6b438c055847cf41e4a247a
expires: Mon, 01 May 2023 08:29:24 GMT
source-age: 5
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"638e5f014e10760fc1deaf86440990400c1ffdeb529f046139de9a428be6e869"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: D87C:0A43:109DCC0:1147932:644F7732
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */1336
date: Mon, 01 May 2023 08:24:24 GMT
via: 1.1 varnish
x-served-by: cache-lhr7352-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929464.161055,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: d8bdf3c1e4ebbfc7365e953af75f7a4a79fc47e2
expires: Mon, 01 May 2023 08:29:24 GMT
source-age: 5
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"638e5f014e10760fc1deaf86440990400c1ffdeb529f046139de9a428be6e869"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: D87C:0A43:109DCC0:1147932:644F7732
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */1336
date: Mon, 01 May 2023 08:24:24 GMT
via: 1.1 varnish
x-served-by: cache-lhr7349-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929464.162316,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: a9d5db82f0ae4e3364bd629ceb0ad33549938e28
expires: Mon, 01 May 2023 08:29:24 GMT
source-age: 5
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"638e5f014e10760fc1deaf86440990400c1ffdeb529f046139de9a428be6e869"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: D87C:0A43:109DCC0:1147932:644F7732
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */1336
date: Mon, 01 May 2023 08:24:24 GMT
via: 1.1 varnish
x-served-by: cache-lhr7343-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929464.162466,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: fc85854c87208e1fdc5205e6259b5a2ba96dd310
expires: Mon, 01 May 2023 08:29:24 GMT
source-age: 5
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"638e5f014e10760fc1deaf86440990400c1ffdeb529f046139de9a428be6e869"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: D87C:0A43:109DCC0:1147932:644F7732
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */1336
date: Mon, 01 May 2023 08:24:24 GMT
via: 1.1 varnish
x-served-by: cache-lhr7359-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929464.161092,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: d58c6f5e7d81a1eab456f2f3544c946855dda7b7
expires: Mon, 01 May 2023 08:29:24 GMT
source-age: 5
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"638e5f014e10760fc1deaf86440990400c1ffdeb529f046139de9a428be6e869"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: D87C:0A43:109DCC0:1147932:644F7732
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */1336
date: Mon, 01 May 2023 08:24:24 GMT
via: 1.1 varnish
x-served-by: cache-lhr7371-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929464.160981,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: fff2089b2bbf5f627a48780ccdc4e78f3889be3b
expires: Mon, 01 May 2023 08:29:24 GMT
source-age: 5
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"638e5f014e10760fc1deaf86440990400c1ffdeb529f046139de9a428be6e869"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: D87C:0A43:109DCC0:1147932:644F7732
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */1336
date: Mon, 01 May 2023 08:24:24 GMT
via: 1.1 varnish
x-served-by: cache-lhr7330-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929464.163576,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: ef5bba9114564687df0c193d7d13e0daf8ebb435
expires: Mon, 01 May 2023 08:29:24 GMT
source-age: 5
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"638e5f014e10760fc1deaf86440990400c1ffdeb529f046139de9a428be6e869"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: D87C:0A43:109DCC0:1147932:644F7732
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */1336
date: Mon, 01 May 2023 08:24:24 GMT
via: 1.1 varnish
x-served-by: cache-lhr7383-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929464.212536,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 2d77b47db4bf17057d2f279f633994d1b2078a1c
expires: Mon, 01 May 2023 08:29:24 GMT
source-age: 5
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"638e5f014e10760fc1deaf86440990400c1ffdeb529f046139de9a428be6e869"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: D87C:0A43:109DCC0:1147932:644F7732
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */1336
date: Mon, 01 May 2023 08:24:29 GMT
via: 1.1 varnish
x-served-by: cache-lhr7371-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929470.894621,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 0c398bcb47300886a0cac84307dbe094fdf5f1ac
expires: Mon, 01 May 2023 08:29:29 GMT
source-age: 11
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"638e5f014e10760fc1deaf86440990400c1ffdeb529f046139de9a428be6e869"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: D87C:0A43:109DCC0:1147932:644F7732
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */1336
date: Mon, 01 May 2023 08:24:29 GMT
via: 1.1 varnish
x-served-by: cache-lhr7325-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929470.941219,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: d43fecc98e0f2b38073baef33b706e50efdaba66
expires: Mon, 01 May 2023 08:29:29 GMT
source-age: 11
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"638e5f014e10760fc1deaf86440990400c1ffdeb529f046139de9a428be6e869"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: D87C:0A43:109DCC0:1147932:644F7732
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */1336
date: Mon, 01 May 2023 08:24:29 GMT
via: 1.1 varnish
x-served-by: cache-lhr7330-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929470.941051,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 64c95498808fed1b418bbc5432a3d2dc7cc0bb0d
expires: Mon, 01 May 2023 08:29:29 GMT
source-age: 11
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"638e5f014e10760fc1deaf86440990400c1ffdeb529f046139de9a428be6e869"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: D87C:0A43:109DCC0:1147932:644F7732
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */1336
date: Mon, 01 May 2023 08:24:29 GMT
via: 1.1 varnish
x-served-by: cache-lhr7387-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929470.944076,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: aec12ca58ef409a386e0ce620768832a7a065290
expires: Mon, 01 May 2023 08:29:29 GMT
source-age: 11
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"638e5f014e10760fc1deaf86440990400c1ffdeb529f046139de9a428be6e869"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: D87C:0A43:109DCC0:1147932:644F7732
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */1336
date: Mon, 01 May 2023 08:24:29 GMT
via: 1.1 varnish
x-served-by: cache-lhr7348-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929470.955115,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: e0ff030f57773d1451f39af3b203bf792a913f6d
expires: Mon, 01 May 2023 08:29:29 GMT
source-age: 11
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"638e5f014e10760fc1deaf86440990400c1ffdeb529f046139de9a428be6e869"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: D87C:0A43:109DCC0:1147932:644F7732
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */1336
date: Mon, 01 May 2023 08:24:29 GMT
via: 1.1 varnish
x-served-by: cache-lhr7328-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929470.955137,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 3761c450ba1281628d93c80f8cb64ecbfdf7a4dc
expires: Mon, 01 May 2023 08:29:29 GMT
source-age: 11
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"638e5f014e10760fc1deaf86440990400c1ffdeb529f046139de9a428be6e869"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: D87C:0A43:109DCC0:1147932:644F7732
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */1336
date: Mon, 01 May 2023 08:24:29 GMT
via: 1.1 varnish
x-served-by: cache-lhr7377-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929470.955192,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 16af2163f18c36ec02799fbf7b99b388892a4953
expires: Mon, 01 May 2023 08:29:29 GMT
source-age: 11
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"638e5f014e10760fc1deaf86440990400c1ffdeb529f046139de9a428be6e869"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: D87C:0A43:109DCC0:1147932:644F7732
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */1336
date: Mon, 01 May 2023 08:24:29 GMT
via: 1.1 varnish
x-served-by: cache-lhr7337-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929470.955323,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 5aa4f39a62f3bcd73f5f1d72718b6add41ecfc81
expires: Mon, 01 May 2023 08:29:29 GMT
source-age: 11
content-length: 0

