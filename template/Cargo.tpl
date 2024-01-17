[package]
authors = ["{author} <{email}>"]
build = "{build}"
categories = ["{categories}"]
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
    "/benches/**",
    "/build.rs",
    "/Cargo.toml",
    "/CONTRIBUTING.md",
    "/examples/**",
    "/LICENSE-APACHE",
    "/LICENSE-MIT",
    "/README.md",
    "/src/**",
    "/tests/**"
]

[[bench]]
name = "benchmark"
harness = false
path = "benches/criterion.rs"

[profile.bench]
debug = true

[dependencies]
anyhow = "1.0.79"
dtt = "0.0.5"
serde = { version = "1.0.195", features = ["derive"] }
serde_json = "1.0.111"
serde_yaml = "0.9.30"
toml = "0.8.8"
vrd = "0.0.5"
rlg = "0.0.2"

[dev-dependencies]
criterion = "0.5.1"

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
codegen-units = 1
debug = false
debug-assertions = false
incremental = false
lto = true
opt-level = "s"
overflow-checks = false
panic = "abort"
rpath = false
strip = "symbols"

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
