[package]
authors = ["{author} <{email}>"]
build = "{build}"
categories = {categories}
description = "{description}"
documentation = "{documentation}"
edition = "{edition}"
exclude = [
    "/.git/*",
    "/.github/*",
    "/.gitignore",
    "/.vscode/*"
    ]
homepage = "{homepage}"
keywords = ["{keywords}"]
license = "{license}"
name = "{name}"
readme = "{readme}"
repository = "{repository}"
rust-version = "{rustversion}"
version = "{version}"
include = [
    "../../CONTRIBUTING.md",
    "../../LICENSE-APACHE",
    "../../LICENSE-MIT",
    "/benches/**",
    "/build.rs",
    "/Cargo.toml",
    "/examples/**",
    "/README.md",
    "/src/**",
    "/tests/**",
]

[[bench]]
name = "benchmark"
harness = false
path = "benches/bench.rs"

[profile.bench]
debug = true

[dependencies]
serde = { version = "1.0.152", features = ["derive"] }

[dev-dependencies]
criterion = "0.4.0"

[lib]
crate-type = ["lib"]
name = "{name}"
path = "src/lib.rs"

[features]
default = []

[package.metadata.docs.rs]
all-features = true

[profile.dev]
codegen-units = 256
debug = true
debug-assertions = true
incremental = true
lto = false
opt-level = 0
overflow-checks = true
panic = 'unwind'
rpath = false
strip = false

[profile.release]
codegen-units = 1        # Compile crates one after another so the compiler can optimize better
debug = false            # Disable debug information
debug-assertions = false # Disable debug assertions
incremental = false      # Disable incremental compilation
lto = true               # Enables link to optimizations
opt-level = "s"          # Optimize for binary size
overflow-checks = false  # Disable overflow checks
panic = "abort"          # Strip expensive panic clean-up logic
rpath = false            # Disable rpath
strip = "symbols"        # Automatically strip symbols from the binary.

[profile.test]
codegen-units = 256
debug = true
debug-assertions = true
incremental = true
lto = false
opt-level = 0
overflow-checks = true
rpath = false
strip = false
HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:21:08 GMT
via: 1.1 varnish
x-served-by: cache-lhr7354-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929268.996278,VS0,VE166
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 4c66cdcf2b3235fe40d2215a4bb4cf9ae3b35fbb
expires: Mon, 01 May 2023 08:26:08 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:21:08 GMT
via: 1.1 varnish
x-served-by: cache-lhr7365-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929268.997863,VS0,VE164
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: f4517ffb637c8571e6fd166a495934f58f7d1554
expires: Mon, 01 May 2023 08:26:08 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:21:08 GMT
via: 1.1 varnish
x-served-by: cache-lhr7362-LHR
x-cache: MISS
x-cache-hits: 0
x-timer: S1682929268.981333,VS0,VE180
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: e715981719af8b43a68de6218838ae04df9aff8f
expires: Mon, 01 May 2023 08:26:08 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:21:08 GMT
via: 1.1 varnish
x-served-by: cache-lhr7342-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929268.996184,VS0,VE166
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 79afad0a926093427213e13d4fccab483a125e32
expires: Mon, 01 May 2023 08:26:08 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:21:08 GMT
via: 1.1 varnish
x-served-by: cache-lhr7371-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929268.996303,VS0,VE166
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: e9422232c15f76cdc72b9d6055f287085f784329
expires: Mon, 01 May 2023 08:26:08 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:21:08 GMT
via: 1.1 varnish
x-served-by: cache-lhr7375-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929268.996564,VS0,VE178
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 237ff33503e1d3a0317f1f3b0dbd135320a5b138
expires: Mon, 01 May 2023 08:26:08 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:21:31 GMT
via: 1.1 varnish
x-served-by: cache-lhr7373-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929292.796638,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: da56da934e40a9e627cdd31e85ded90c2856ae0e
expires: Mon, 01 May 2023 08:26:31 GMT
source-age: 24
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:21:31 GMT
via: 1.1 varnish
x-served-by: cache-lhr7359-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929292.803242,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 771a89ad814e2c21b0d00680ee84f1b29f694e91
expires: Mon, 01 May 2023 08:26:31 GMT
source-age: 24
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:21:31 GMT
via: 1.1 varnish
x-served-by: cache-lhr7343-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929292.805985,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 2827dd2b430c2924015039629cc867028f289599
expires: Mon, 01 May 2023 08:26:31 GMT
source-age: 24
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:21:31 GMT
via: 1.1 varnish
x-served-by: cache-lhr7366-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929292.803340,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: e253494751a75ba5e566912c9fbe93a1c79d1b52
expires: Mon, 01 May 2023 08:26:31 GMT
source-age: 24
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:21:31 GMT
via: 1.1 varnish
x-served-by: cache-lhr7356-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929292.803341,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 4e1a8cda182f27562455bb19f95ff9851492df8d
expires: Mon, 01 May 2023 08:26:31 GMT
source-age: 24
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:21:31 GMT
via: 1.1 varnish
x-served-by: cache-lhr7367-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929292.803300,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 435f09120d15bb9da3d89d4f668abcc68d44c51e
expires: Mon, 01 May 2023 08:26:31 GMT
source-age: 24
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:24:17 GMT
via: 1.1 varnish
x-served-by: cache-lhr7363-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929457.135456,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 89cf6eca17ff94c65efe6be13d90f8fedb79e490
expires: Mon, 01 May 2023 08:29:17 GMT
source-age: 189
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:24:17 GMT
via: 1.1 varnish
x-served-by: cache-lhr7329-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929457.135459,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 862ef6e3231363ae53d404fb09a1d38e43eba6d1
expires: Mon, 01 May 2023 08:29:17 GMT
source-age: 189
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:24:17 GMT
via: 1.1 varnish
x-served-by: cache-lhr7351-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929457.135426,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 2cdae27e7d0f19dfe96cc7dc02ca16815e18e676
expires: Mon, 01 May 2023 08:29:17 GMT
source-age: 189
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:24:17 GMT
via: 1.1 varnish
x-served-by: cache-lhr7360-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929457.141270,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 07d09b3822d676bdeb5122fca638ea024beff5cf
expires: Mon, 01 May 2023 08:29:17 GMT
source-age: 189
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:24:17 GMT
via: 1.1 varnish
x-served-by: cache-lhr7320-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929457.147725,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 83d532812d766efdb7de4d83361b6d67e89119a8
expires: Mon, 01 May 2023 08:29:17 GMT
source-age: 189
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:24:17 GMT
via: 1.1 varnish
x-served-by: cache-lhr7353-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929457.147759,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: c5dacb8173f0b6c8237b3cc50a2a77fa07682562
expires: Mon, 01 May 2023 08:29:17 GMT
source-age: 189
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:24:22 GMT
via: 1.1 varnish
x-served-by: cache-lhr7362-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929462.424440,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: cb78a1d5883398438759544f9fec92b436519d11
expires: Mon, 01 May 2023 08:29:22 GMT
source-age: 194
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:24:22 GMT
via: 1.1 varnish
x-served-by: cache-lhr7354-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929462.437295,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: e4247e35525acd180666dcde0ce003f78113f354
expires: Mon, 01 May 2023 08:29:22 GMT
source-age: 194
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:24:22 GMT
via: 1.1 varnish
x-served-by: cache-lhr7348-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929462.437410,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 2094d347799af9958f13131d710c85f5386f6e2b
expires: Mon, 01 May 2023 08:29:22 GMT
source-age: 194
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:24:22 GMT
via: 1.1 varnish
x-served-by: cache-lhr7359-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929462.437347,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 0fd103654f6193d27636bf5ecacdc26addab9905
expires: Mon, 01 May 2023 08:29:22 GMT
source-age: 194
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:24:22 GMT
via: 1.1 varnish
x-served-by: cache-lhr7331-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929462.444068,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 59d8b69a0ba9f12d25584ed166e03caf55c5131d
expires: Mon, 01 May 2023 08:29:22 GMT
source-age: 194
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:24:22 GMT
via: 1.1 varnish
x-served-by: cache-lhr7320-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929462.444596,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 1ede6c6cc8bb9351f7b30b35a87daf122e30f0d1
expires: Mon, 01 May 2023 08:29:22 GMT
source-age: 194
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:24:22 GMT
via: 1.1 varnish
x-served-by: cache-lhr7342-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929462.444244,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 49f8560f11505b7284ee028eefbb414e1498bec9
expires: Mon, 01 May 2023 08:29:22 GMT
source-age: 194
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:24:22 GMT
via: 1.1 varnish
x-served-by: cache-lhr7331-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929462.470675,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 07aa47623b08635595c622f843aae254d149b1d5
expires: Mon, 01 May 2023 08:29:22 GMT
source-age: 194
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:24:28 GMT
via: 1.1 varnish
x-served-by: cache-lhr7378-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929468.196604,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: ddad2b416abe3233b143a6414ccb21947a7d0cad
expires: Mon, 01 May 2023 08:29:28 GMT
source-age: 200
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:24:28 GMT
via: 1.1 varnish
x-served-by: cache-lhr7341-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929468.223518,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 6c8b9654f68f9400cac96403759d7c4fc7efb64e
expires: Mon, 01 May 2023 08:29:28 GMT
source-age: 200
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:24:28 GMT
via: 1.1 varnish
x-served-by: cache-lhr7358-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929468.230722,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 51891bfef239d03377844837f891338284b5f883
expires: Mon, 01 May 2023 08:29:28 GMT
source-age: 200
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:24:28 GMT
via: 1.1 varnish
x-served-by: cache-lhr7384-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929468.230572,VS0,VE2
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 4e120750480d28a6c623d4d738f83e177422fa37
expires: Mon, 01 May 2023 08:29:28 GMT
source-age: 200
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:24:28 GMT
via: 1.1 varnish
x-served-by: cache-lhr7344-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929468.230740,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 5349180e102eb9892920cfd053e6c747627e3fdd
expires: Mon, 01 May 2023 08:29:28 GMT
source-age: 200
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:24:28 GMT
via: 1.1 varnish
x-served-by: cache-lhr7332-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929468.230619,VS0,VE2
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 9ef618b10defbbc4721338b4b0e0efc2f8102133
expires: Mon, 01 May 2023 08:29:28 GMT
source-age: 200
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:24:28 GMT
via: 1.1 varnish
x-served-by: cache-lhr7346-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929468.234929,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: ec1e7b48d724092ff2d776c1b9caeb41e57979da
expires: Mon, 01 May 2023 08:29:28 GMT
source-age: 200
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"a178b8c777865f881c291a83510e0bfdaebac5c203cf6c438f2804f6b1a5f647"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 6D58:1AD0:1BCCAAB:1CE0E78:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */821
date: Mon, 01 May 2023 08:24:28 GMT
via: 1.1 varnish
x-served-by: cache-lhr7369-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929468.250889,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: efc4bedf1df5ffc2fdf944aeea5db33dacbc5e0f
expires: Mon, 01 May 2023 08:29:28 GMT
source-age: 200
content-length: 0

