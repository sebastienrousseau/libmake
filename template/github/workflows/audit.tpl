name: 🧪 Audit

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
  dependencies:
    name: Audit dependencies
    runs-on: ubuntu-latest
    steps:
      - uses: hecrj/setup-rust-action@v2
      - name: Install cargo-audit
        run: cargo install cargo-audit

      - uses: actions/checkout@v4
      - name: Resolve dependencies
        run: cargo update

      - name: Audit vulnerabilities
        run: cargo audit
