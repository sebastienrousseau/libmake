[package]
authors = ["{author} <{email}>"]            # Add your name and email here.
build = "{build}"                           # Add your build command here (e.g. build.rs).
categories = {categories}                   # Add your categories here. See https://doc.rust-lang.org/cargo/reference/manifest.html?highlight=keywords#the-categories-field for more information.
description = "{description}"               # Add your description here.
documentation = "{documentation}"           # Add your documentation link here.
edition = "{edition}"                       # Add your edition here (e.g. 2018).
exclude = [
    "/.git/*",
    "/.github/*",
    "/.gitignore",
    "/.vscode/*"
    ]                                       # Add files to exclude here
homepage = "{homepage}"                     # Add your homepage here
keywords = ["{keywords}"]                   # Add your keywords here. See https://doc.rust-lang.org/cargo/reference/manifest.html?highlight=keywords#the-keywords-field for more information.
license = "{license}"                       # Add your license here.
name = "{name}"                             # Add your library name here.
readme = "{readme}"                         # Add your readme here.
repository = "{repository}"                 # Add your repository here.
rust-version = "{rustversion}"              # Add your rust version here.
version = "{version}"                       # Add your version here.
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
    "/tests/**",
    "/xtask/**",
]                                           # Add files to include here.

[[bench]]
name = "benchmark"
harness = false
path = "benches/criterion.rs"

[profile.bench]
debug = true

[dependencies]
serde = { version = "1.0.162", features = ["derive"] }
serde_json = "1.0.96"

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
codegen-units = 256                         # Number of parallel codegen units
debug = true                                # Enable debug symbols
debug-assertions = true                     # Enable debug assertions
incremental = true                          # Enable incremental compilation
lto = false                                 # Disable link-time optimization
opt-level = 0                               # Optimize for speed
overflow-checks = true                      # Enable overflow checks
panic = 'unwind'                            # Use unwinding panic handling
rpath = false                               # Disable rpath
strip = false                               # Disable symbol stripping

[profile.release]
codegen-units = 1                           # Number of parallel codegen units
debug = false                               # Disable debug symbols
debug-assertions = false                    # Disable debug assertions
incremental = false                         # Disable incremental compilation
lto = true                                  # Enable link-time optimization
opt-level = "s"                             # Optimize for size
overflow-checks = false                     # Disable overflow checks
panic = "abort"                             # Use aborting panic handling
rpath = false                               # Disable rpath
strip = "symbols"                           # Strip symbols

[profile.test]
codegen-units = 256                         # Number of parallel codegen units
debug = true                                # Enable debug symbols
debug-assertions = true                     # Enable debug assertions
incremental = true                          # Enable incremental compilation
lto = false                                 # Disable link-time optimization
opt-level = 0                               # Optimize for speed
overflow-checks = true                      # Enable overflow checks
rpath = false                               # Disable rpath
strip = false                               # Disable symbol stripping