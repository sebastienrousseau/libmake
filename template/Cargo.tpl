[package]
# Metadata about the package.
authors = "{author}"
build = "{build}"
categories = [{categories}]
description = "{description}"
documentation = "{documentation}"
edition = "{edition}"
exclude = ["/.git/*", "/.github/*", "/.gitignore", "/.vscode/*"]
homepage = "{homepage}"
keywords = [{keywords}]
license = "{license}"
name = "{name}"
readme = "{readme}"
repository = "{repository}"
rust-version = "{rustversion}"
version = "{version}"
include = [
    "/CONTRIBUTING.md",
    "/LICENSE-APACHE",
    "/LICENSE-MIT",
    "/benches/**",
    "/build.rs",
    "/Cargo.toml",
    "/examples/**",
    "/README.md",
    "/src/**",
    "/tests/**"
]

[[bench]]
# Benchmarking configuration.
name = "benchmark"
harness = false
path = "benches/criterion.rs"

[profile.bench]
debug = true

[dependencies]
# The dependencies of the package.
anyhow = "1.0.83"
dtt = "0.0.6"
env_logger = "0.11.3"
rlg = "0.0.4"
serde = { version = "1.0.201", features = ["derive"] }
serde_json = "1.0.117"
serde_yml = "0.0.5"
toml = "0.8.12"
vrd = "0.0.7"

[dev-dependencies]
# The development dependencies of the package.
criterion = "0.5.1"

[lib]
# The library configuration.
crate-type = ["lib"]
name = "{name}"
path = "src/lib.rs"

[features]
# The features of the package.
default = []

[package.metadata.docs.rs]
# Configuration for docs.rs.
targets = ["x86_64-unknown-linux-gnu"]
rustdoc-args = ["--generate-link-to-definition"]

# Linting config
[lints.rust]

## Warn
missing_copy_implementations = "warn"
missing_docs = "warn"
unstable_features = "warn"
unused_extern_crates = "warn"
unused_results = "warn"

## Allow
bare_trait_objects = "allow"
elided_lifetimes_in_paths = "allow"
non_camel_case_types = "allow"
non_upper_case_globals = "allow"
trivial_bounds = "allow"
unsafe_code = "allow"

## Forbid
missing_debug_implementations = "forbid"
non_ascii_idents = "forbid"
unreachable_pub = "forbid"

## Deny
dead_code = "deny"
deprecated_in_future = "deny"
ellipsis_inclusive_range_patterns = "deny"
explicit_outlives_requirements = "deny"
future_incompatible = { level = "deny", priority = -1 }
keyword_idents = "deny"
macro_use_extern_crate = "deny"
meta_variable_misuse = "deny"
missing_fragment_specifier = "deny"
noop_method_call = "deny"
pointer_structural_match = "deny"
rust_2018_idioms = { level = "deny", priority = -1 }
rust_2021_compatibility = { level = "deny", priority = -1 }
single_use_lifetimes = "deny"
trivial_casts = "deny"
trivial_numeric_casts = "deny"
unused = { level = "deny", priority = -1 }
unused_features = "deny"
unused_import_braces = "deny"
unused_labels = "deny"
unused_lifetimes = "deny"
unused_macro_rules = "deny"
unused_qualifications = "deny"
variant_size_differences = "deny"

[package.metadata.clippy]
# Clippy configuration.
warn-lints = ["clippy::all", "clippy::pedantic", "clippy::cargo", "clippy::nursery"]

[profile.dev]
# Development profile configuration.
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
# Release profile configuration.
codegen-units = 1
debug = false
debug-assertions = false
incremental = false
lto = true
opt-level = "s"
overflow-checks = false
panic = 'abort'
rpath = false
strip = 'symbols'

[profile.test]
# Test profile configuration.
codegen-units = 256
debug = true
debug-assertions = true
incremental = true
lto = false
opt-level = 0
overflow-checks = true
rpath = false
strip = false
