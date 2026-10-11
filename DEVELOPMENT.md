# Developing libmake

```bash
git clone https://github.com/sebastienrousseau/libmake
cd libmake
cargo test
```

- Format and lint before pushing: `cargo fmt --all -- --check` and
  `cargo clippy --all-targets -- -D warnings`.
- Every behaviour change lands with its test in the same commit.
- Conventional commits; releases follow SemVer with a
  Keep-a-Changelog entry per release.
- CI (`.github/workflows/ci.yml`) is the merge gate; keep it green.
