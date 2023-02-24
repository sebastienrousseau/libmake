[package]
authors = ["{author} <{email}>}"]
categories = {categories}
description = "{description}"
edition = "2021"
keywords = {keywords}
license = "{license}"
name = "{name}"
repository = "{repository}"
rust-version = "{rustversion}"
version = "{version}"
include = [
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

# [[bench]]
# name = "benchmark"
# harness = false
# path = "benches/bench.rs"

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
