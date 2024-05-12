name: 🧪 Lint

on:
  push:
    branches:
      - feat/{name}
  pull_request:
    branches:
      - feat/{name}
  release:
    types: [created]

jobs:
  all:
    name: Lint
    runs-on: ubuntu-latest
    steps:
      - uses: hecrj/setup-rust-action@v2
        with:
          components: clippy
      - uses: actions/checkout@v4
      - name: Check lints
        run: cargo clippy --workspace --all-features --all-targets --no-deps -- -D warnings
