[package]
authors = [{author}]
build = {build}
categories = {categories}
description = {description}
documentation = {documentation}
edition = {edition}
exclude = ["/.git/*", "/.github/*", "/.gitignore", "/.vscode/*"]
homepage = {homepage}
keywords = {keywords}
license = {license}
name = "{name}"
readme = {readme}
repository = "{repository}"
rust-version = {rustversion}
version = {version}
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
name = "benchmark"
harness = false
path = "benches/criterion.rs"

[profile.bench]
debug = true

[dependencies]
anyhow = "1.0.81"
dtt = "0.0.5"
env_logger = "0.11.3"
rlg = "0.0.3"
serde = { version = "1.0.197", features = ["derive"] }
serde_json = "1.0.115"
serde_yaml = "0.9.33"
toml = "0.8.12"
vrd = "0.0.6"

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

# Linting config
[lints.rust]

## Forbid
missing_debug_implementations = "forbid"
missing_docs = "warn"
non_ascii_idents = "forbid"
unreachable_pub = "forbid"
unsafe_code = "forbid"

## Deny
dead_code = "deny"
deprecated_in_future = "deny"
ellipsis_inclusive_range_patterns = "deny"
explicit_outlives_requirements = "deny"
future_incompatible = "deny"
keyword_idents = "deny"
macro_use_extern_crate = "deny"
meta_variable_misuse = "deny"
missing_fragment_specifier = "deny"
noop_method_call = "deny"
pointer_structural_match = "deny"
rust_2018_idioms = "deny"
rust_2021_compatibility = "deny"
single_use_lifetimes = "deny"
trivial_casts = "deny"
trivial_numeric_casts = "deny"
unused = "deny"
unused_features = "deny"
unused_import_braces = "deny"
unused_labels = "deny"
unused_lifetimes = "deny"
unused_macro_rules = "deny"
unused_qualifications = "deny"
variant_size_differences = "deny"

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
