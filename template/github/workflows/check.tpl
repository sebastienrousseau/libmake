name: 🧪 Check

on:
  push:
    branches:
      - main
      - feat/{name}
  pull_request:
    branches:
      - feat/{name}
  release:
    types: [created]

jobs:
  all:
    name: Check
    runs-on: ubuntu-latest
    steps:
      - uses: hecrj/setup-rust-action@v2
        with:
          components: clippy
      - uses: actions/checkout@v4
      - name: Check lints
        run: cargo check --all-targets --workspace --all-features
