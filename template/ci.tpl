name: ❯ {name} release

on:
  pull_request:
    branches:
      - main
      - 'feat/*'
  push:
    branches:
      - main
      - 'feat/*'

jobs:
  # This job checks a local package and all of its dependencies for
  # errors.
  check:
    name: ❯ Check 💵
    runs-on: ubuntu-latest
    strategy:
      fail-fast: false
      matrix:
        include:
          - rust: stable
            target: x86_64-unknown-linux-gnu
            os: ubuntu-latest

    steps:
      # Check out the repository code
      - name: Checkout sources
        id: checkout
        uses: actions/checkout@v3

      # Install the stable Rust toolchain
      - name: Install stable toolchain
        id: install-toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true

      # Cache dependencies to speed up subsequent builds
      - name: Cache dependencies
        id: cache-dependencies
        uses: actions/cache@v3
        with:
          path: ~/.cargo
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: ${{ runner.os }}-cargo-

      # Run cargo check to check for errors
      - uses: actions-rs/cargo@v1
        with:
          command: check
          args: --all-targets --workspace --all-features

  # This job runs the tests for the project.
  test:
    name: ❯ Test 🧪
    runs-on: ubuntu-latest
    strategy:
      fail-fast: false
      matrix:
        include:
          - rust: stable
            target: x86_64-unknown-linux-gnu

    steps:
      # Check out the repository code
      - name: Checkout sources
        id: checkout
        uses: actions/checkout@v3

      # Install the stable Rust toolchain
      - name: Install stable toolchain
        id: install-toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: ${{ matrix.rust }}

      # Cache dependencies to speed up subsequent builds
      - name: Cache dependencies
        id: cache-dependencies
        uses: actions/cache@v3
        with:
          path: ~/.cargo
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: ${{ runner.os }}-cargo-

      # Run cargo hack to check for errors
      - name: Install cargo-hack
        uses: taiki-e/install-action@cargo-hack
        id: install-cargo-hack

      - run: cargo test --all-targets --workspace --all-features

  # This job runs the tests for the project.
  coverage:
    name: ❯ Coverage 📊
    if: github.ref == 'refs/heads/main' && github.event_name == 'push'
    runs-on: ubuntu-latest
    strategy:
      matrix:
        include:
          - rust: stable
            target: x86_64-unknown-linux-gnu

    steps:
      # Check out the repository code
      - name: Checkout sources
        id: checkout
        uses: actions/checkout@v3

      # Install the stable Rust toolchain
      - name: Install stable toolchain
        id: install-toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: ${{ matrix.rust }}
          override: true
          components: llvm-tools-preview

      # Cache dependencies to speed up subsequent builds
      - name: Cache dependencies
        id: cache-dependencies
        uses: actions/cache@v3
        with:
          path: ~/.cargo
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: ${{ runner.os }}-cargo-

      # Install grcov
      - name: Install grcov
        run: |
          mkdir -p "${HOME}/.local/bin"
          curl -sL https://github.com/mozilla/grcov/releases/download/v0.8.13/grcov-x86_64-unknown-linux-gnu.tar.bz2 | tar jxf - -C "${HOME}/.local/bin"
          echo "$HOME/.local/bin" >> $GITHUB_PATH

      # Use grcov to generate a coverage report
      - name: Generate coverage report
        id: coverage
        uses: actions-rs/cargo@v1
        with:
          command: xtask
          args: coverage

      # Upload the coverage report to codecov
      - name: Upload coverage report to codecov
        id: codecov
        uses: codecov/codecov-action@v3
        with:
          files: coverage/*.lcov

  lints:
    name: ❯ Lints 🧹
    runs-on: ubuntu-latest
    strategy:
      matrix:
        include:
          - rust: stable
            target: x86_64-unknown-linux-gnu
    steps:
      # Check out the repository code
      - name: Checkout sources
        id: checkout
        uses: actions/checkout@v3
        with:
          submodules: true

      # Install the stable Rust toolchain
      - name: Install stable toolchain
        uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
          override: true
          components: clippy

      # Cache dependencies to speed up subsequent builds
      - name: Cache dependencies
        uses: actions/cache@v3
        with:
          path: ~/.cargo
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: ${{ runner.os }}-cargo-

      # Run cargo clippy to check for linting errors
      - name: Run cargo clippy
        if: github.ref == !github.event.check_run.conclusion
        uses: actions-rs/cargo@v1
        with:
          command: clippy
          args: --all-targets --all-features -- -D warnings

      # Run Cargo Format for the code style
      - name: Run Cargo Format
        id: run-check-format
        if: github.ref == !github.event.check_run.conclusion
        run: |
          cargo check --all --all-features --workspace --verbose

      # Run cargo clippy to check for linting errors
      - name: Run Clippy
        id: run-check-clippy
        if: github.ref == !github.event.check_run.conclusion
        run: |
          cargo clippy --all-targets --all-features --workspace -- -D warnings


  build:
    # This job builds the project for all the targets and generates a
    # release artifact that contains the binaries for all the targets.
    name: ❯ Build 🛠
    if: github.ref == 'refs/heads/main' && github.event_name == 'push'
    strategy:
      fail-fast: false
      matrix:
        target:
          # List of targets: https://doc.rust-lang.org/nightly/rustc/platform-support.html

          # FreeBSD targets 🐬
          - x86_64-unknown-freebsd # 64-bit FreeBSD on x86-64                                     ✅ Tested

          # Linux targets 🐧
          - aarch64-unknown-linux-gnu # 64-bit Linux systems on ARM architecture                  ✅ Tested
          - aarch64-unknown-linux-musl # 64-bit Linux systems on ARM architecture                 ✅ Tested
          - arm-unknown-linux-gnueabi # ARMv6 Linux (kernel 3.2, glibc 2.17)                      ✅ Tested
          - armv7-unknown-linux-gnueabihf # ARMv7 Linux, hardfloat (kernel 3.2, glibc 2.17)       ✅ Tested
          - i686-unknown-linux-gnu # 32-bit Linux (kernel 3.2+, glibc 2.17+)                      ✅ Tested
          - i686-unknown-linux-musl # 32-bit Linux (kernel 3.2+, musl libc)                       ✅ Tested
          - x86_64-unknown-linux-gnu # 64-bit Linux (kernel 2.6.32+, glibc 2.11+)                 ✅ Tested
          - x86_64-unknown-linux-musl # 64-bit Linux (kernel 2.6.32+, musl libc)                  ✅ Tested

          # macOS targets 🍎
          - aarch64-apple-darwin # 64-bit macOS on Apple Silicon                                  ✅ Tested
          - x86_64-apple-darwin # 64-bit macOS (10.7 Lion or later)                               ✅ Tested

          # Illumos targets 🌞
          - x86_64-unknown-illumos # 64-bit Illumos on x86-64                                     ✅ Tested

        include:
          # FreeBSD targets 🐬
          - target: x86_64-unknown-freebsd
            os: ubuntu-latest
            cross: true

          # Linux targets 🐧
          - target: aarch64-unknown-linux-gnu
            os: ubuntu-latest
            cross: true
          - target: aarch64-unknown-linux-musl
            os: ubuntu-latest
            cross: true
          - target: arm-unknown-linux-gnueabi
            os: ubuntu-latest
            cross: true
          - target: armv7-unknown-linux-gnueabihf
            os: ubuntu-latest
            cross: true
          - target: i686-unknown-linux-gnu
            os: ubuntu-latest
            cross: true
          - target: i686-unknown-linux-musl
            os: ubuntu-latest
            cross: true
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
            cross: true
          - target: x86_64-unknown-linux-musl
            os: ubuntu-latest
            cross: true

          # Illumos targets 🌞
          - target: x86_64-unknown-illumos
            os: ubuntu-latest
            cross: true

          # macOS targets 🍎
          - target: aarch64-apple-darwin
            os: macos-latest
            cross: true
          - target: x86_64-apple-darwin
            os: macos-latest
            cross: true

    runs-on: ${{ matrix.os }}

    steps:
      # Check out the repository code
      - name: Checkout sources
        id: checkout
        uses: actions/checkout@v3

      # Install the stable Rust toolchain
      - name: Install stable toolchain
        id: install-toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true

      # Cache dependencies to speed up subsequent builds
      - name: Cache dependencies
        id: cache-dependencies
        uses: actions/cache@v3
        with:
          path: ~/.cargo
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: ${{ runner.os }}-cargo-

      # Install the targets for the cross compilation toolchain
      - name: Install target
        id: install-target
        run: rustup target add ${{ matrix.target }}

      # Update the version number based on the Cargo.toml file
      - name: Update version number
        id: update-version
        run: |
          NEW_VERSION=$(grep version Cargo.toml | sed -n 2p | cut -d '"' -f 2)
          echo "VERSION=$NEW_VERSION" >> $GITHUB_ENV
        shell: /bin/bash -e {0}

      # Install the cross compilation toolchain
      - name: Install Cross
        id: install-cross
        run: |
          # Install cross
          cargo install cross
          # Clean the build artifacts
          cargo clean --verbose
        shell: /bin/bash -e {0}

      # Build the targets
      - name: Build targets
        id: build-targets
        uses: actions-rs/cargo@v1
        with:
          use-cross: true
          command: build
          args: --verbose --workspace --release --target ${{ matrix.target }}

      # Package the binary for each target
      - name: Package the binary
        id: package-binary
        run: |
          mkdir -p target/package
          tar czf target/package/${{ matrix.target }}.tar.gz -C target/${{ matrix.target }}/release .
          echo "${{ matrix.target }}.tar.gz=target/package/${{ matrix.target }}.tar.gz" >> $GITHUB_ENV

      # Upload the binary for each target
      - name: Upload the binary
        id: upload-binary
        uses: actions/upload-artifact@v2
        with:
          name: ${{ matrix.target }}.tar.gz
          path: ${{ env[format('{0}.tar.gz', matrix.target)] }}

  # Release the binary to GitHub Releases
  release:
    name: ❯ Release 🚀
    if: github.ref == 'refs/heads/main' && github.event_name == 'push'
    needs: build
    runs-on: ubuntu-latest
    steps:
      # Check out the repository code
      - name: Checkout sources
        uses: actions/checkout@v3

      # Install the stable Rust toolchain
      - name: Install stable toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true

      # Update the version number based on the Cargo.toml file
      - name: Update version number
        run: |
          NEW_VERSION=$(grep version Cargo.toml | sed -n 2p | cut -d '"' -f 2)
          echo "VERSION=$NEW_VERSION" >> $GITHUB_ENV
        shell: /bin/bash -e {0}

      # Cache dependencies to speed up subsequent builds
      - name: Cache dependencies
        uses: actions/cache@v3
        with:
          path: ~/.cargo
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: ${{ runner.os }}-cargo-

      # Download the artifacts from the build job
      - name: Download artifacts
        run: |
          for target in x86_64-unknown-freebsd aarch64-unknown-linux-gnu aarch64-unknown-linux-musl arm-unknown-linux-gnueabi armv7-unknown-linux-gnueabihf i686-unknown-linux-gnu i686-unknown-linux-musl x86_64-unknown-linux-gnu x86_64-unknown-linux-musl aarch64-apple-darwin x86_64-apple-darwin x86_64-unknown-illumos; do
            echo "Downloading $target artifact"
            name="${target}.tar.gz"
            echo "Artifact name: $name"
            mkdir -p target/package
            curl -sSL -H "Authorization: token ${GITHUB_TOKEN}" -H "Accept: application/vnd.github.v3+json" -L "${GITHUB_API_URL}/repos/${GITHUB_REPOSITORY}/actions/runs/${RUN_ID}/artifacts/${name}" -o "target/package/${name}"
          done
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          RUN_ID: ${{ github.event.workflow_run.id }}

      # Generate the changelog based on git log and template file
      - name: Generate Changelog
        run: |
          # Append version information to CHANGELOG.md
          echo "## Release v${{ env.VERSION }} - $(date +'%Y-%m-%d')" >> ${{ github.workspace }}/CHANGELOG.md
          # Copy content of template file to CHANGELOG.md
          cat TEMPLATE.md >> ${{ github.workspace }}/CHANGELOG.md
          # Append git log to CHANGELOG.md
          echo "$(git log --pretty=format:'%s' --reverse $(git describe --tags --abbrev=0)..HEAD)" >> ${{ github.workspace }}/CHANGELOG.md
          # Append empty line to CHANGELOG.md
          echo "" >> ${{ github.workspace }}/CHANGELOG.md

      # Append the artifact links to the changelog
      - name: Append Artifact Links
        run: |
          echo "" >> ${{ github.workspace }}/CHANGELOG.md
          echo "## Artifacts 🎁" >> ${{ github.workspace }}/CHANGELOG.md
          for filename in target/package/*.tar.gz; do
            link="$(basename $filename)"
            echo "* [$link](${{ github.server_url }}/${{ github.repository }}/releases/download/v${{ env.VERSION }}/$link)" >> ${{ github.workspace }}/CHANGELOG.md
          done

      # Create the release on GitHub Releases
      - name: Create Release
        id: create-release
        uses: actions/create-release@v1
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          tag_name: v${{ env.VERSION }}
          release_name: {name} 🦀 v${{ env.VERSION }}
          body_path: ${{ github.workspace }}/CHANGELOG.md
          draft: true
          prerelease: false

  # Publish the release to Crates.io automatically
  crate:
    name: ❯ Crate.io 🦀
    if: github.ref == 'refs/heads/main' && github.event_name == 'push'
    needs: release
    runs-on: ubuntu-latest

    steps:
      # Check out the repository code
      - name: Checkout
        uses: actions/checkout@v3

      # Install the stable Rust toolchain
      - name: Install stable toolchain
        id: install-toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true

      # Cache dependencies to speed up subsequent builds
      - name: Cache dependencies
        id: cache-dependencies
        uses: actions/cache@v2
        with:
          path: /home/runner/.cargo/registry/index/
          key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: ${{ runner.os }}-cargo-index-

      # Update the version number based on the Cargo.toml file
      - name: Update version number
        id: update-version
        run: |
          NEW_VERSION=$(grep version Cargo.toml | sed -n 2p | cut -d '"' -f 2)
          echo "VERSION=$NEW_VERSION" >> $GITHUB_ENV
        shell: /bin/bash -e {0}

      # Publish the Rust library to Crate.io
      - name: Publish Library to Crate.io
        id: publish-library
        uses: actions-rs/cargo@v1
        env:
          CARGO_API_TOKEN: ${{ secrets.CARGO_API_TOKEN }}
        with:
          use-cross: true
          command: publish
          args: --dry-run --verbose --token "${CARGO_API_TOKEN}"
HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:21:08 GMT
via: 1.1 varnish
x-served-by: cache-lhr7349-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929268.447676,VS0,VE171
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 26380e5542f1d38bfafdb5d832b3339cf631f140
expires: Mon, 01 May 2023 08:26:08 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:21:08 GMT
via: 1.1 varnish
x-served-by: cache-lhr7371-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929268.486908,VS0,VE132
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: a0c92465439215cd196dcc1c71704bdd027b6cb0
expires: Mon, 01 May 2023 08:26:08 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:21:08 GMT
via: 1.1 varnish
x-served-by: cache-lhr7341-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929268.447679,VS0,VE171
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 0cb2706af58946d3a78054b6728771d29b8f1462
expires: Mon, 01 May 2023 08:26:08 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:21:08 GMT
via: 1.1 varnish
x-served-by: cache-lhr7358-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929268.456059,VS0,VE163
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 25adb913b1679ad02413dcb87fa5e68aea8ebcfa
expires: Mon, 01 May 2023 08:26:08 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:21:08 GMT
via: 1.1 varnish
x-served-by: cache-lhr7345-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929268.454275,VS0,VE165
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 1763f8d96392a5bd6dc6134fe93f202673413482
expires: Mon, 01 May 2023 08:26:08 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:21:08 GMT
via: 1.1 varnish
x-served-by: cache-lhr7352-LHR
x-cache: MISS
x-cache-hits: 0
x-timer: S1682929268.441350,VS0,VE177
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: de3e0913c8a345857014b4483a3c74a92efd2cc6
expires: Mon, 01 May 2023 08:26:08 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:21:32 GMT
via: 1.1 varnish
x-served-by: cache-lhr7375-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929292.073112,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 0cb317a6dca17dbc666a2c41cba20b3462424ece
expires: Mon, 01 May 2023 08:26:32 GMT
source-age: 23
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:21:32 GMT
via: 1.1 varnish
x-served-by: cache-lhr7335-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929292.073301,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: a32cf3ade6f5410ece541a317937d9d08bbe261f
expires: Mon, 01 May 2023 08:26:32 GMT
source-age: 23
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:21:32 GMT
via: 1.1 varnish
x-served-by: cache-lhr7337-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929292.073144,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 4a792663d3ba23f2197d6af6c879fde7b9e41ca9
expires: Mon, 01 May 2023 08:26:32 GMT
source-age: 23
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:21:32 GMT
via: 1.1 varnish
x-served-by: cache-lhr7342-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929292.073471,VS0,VE2
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: f2b87c856e4be7c0cc57d96b7f36b03347d32ce9
expires: Mon, 01 May 2023 08:26:32 GMT
source-age: 23
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:21:32 GMT
via: 1.1 varnish
x-served-by: cache-lhr7350-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929292.078804,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 63bcd7fe3c9b4c1f353cf84dc497272661a91879
expires: Mon, 01 May 2023 08:26:32 GMT
source-age: 23
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:21:32 GMT
via: 1.1 varnish
x-served-by: cache-lhr7387-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929292.085049,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 5288bbff82b776540a5047d28c3ad445e246391d
expires: Mon, 01 May 2023 08:26:32 GMT
source-age: 23
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
date: Mon, 01 May 2023 08:24:17 GMT
via: 1.1 varnish
x-served-by: cache-lhr7353-LHR
x-cache: HIT
x-cache-hits: 12
x-timer: S1682929457.419290,VS0,VE0
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 4071f4672c50d1edef26c5d888d0a2ab31888e5d
expires: Mon, 01 May 2023 08:29:17 GMT
source-age: 189
content-range: bytes */3111
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:24:17 GMT
via: 1.1 varnish
x-served-by: cache-lhr7323-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929457.419216,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: f7f5c64bbd0a5da73b791f77a7fbe637ef862350
expires: Mon, 01 May 2023 08:29:17 GMT
source-age: 189
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:24:17 GMT
via: 1.1 varnish
x-served-by: cache-lhr7346-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929457.425298,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: a5e9ddf66fccd0307d7b5fe7afad819f6e8a7104
expires: Mon, 01 May 2023 08:29:17 GMT
source-age: 189
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:24:17 GMT
via: 1.1 varnish
x-served-by: cache-lhr7336-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929457.432322,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: af4baf7d4c085d71130750035ad5eefdd8787a59
expires: Mon, 01 May 2023 08:29:17 GMT
source-age: 189
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:24:17 GMT
via: 1.1 varnish
x-served-by: cache-lhr7381-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929457.432305,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: afd4b83b321094e8fd00573856c5545f8375e49f
expires: Mon, 01 May 2023 08:29:17 GMT
source-age: 189
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:24:17 GMT
via: 1.1 varnish
x-served-by: cache-lhr7387-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929457.432289,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: e111c20e87212649818a6e1d8bd68496cac14e4f
expires: Mon, 01 May 2023 08:29:17 GMT
source-age: 189
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:24:22 GMT
via: 1.1 varnish
x-served-by: cache-lhr7393-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929463.701305,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: d6c6027a439b6a45fa86f8fe483dfbbbdf747725
expires: Mon, 01 May 2023 08:29:22 GMT
source-age: 194
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:24:22 GMT
via: 1.1 varnish
x-served-by: cache-lhr7325-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929463.714571,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 2083be268d36e0f8229a1e8613c7eb05ffc06098
expires: Mon, 01 May 2023 08:29:22 GMT
source-age: 194
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:24:22 GMT
via: 1.1 varnish
x-served-by: cache-lhr7351-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929463.714729,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: ee25deacacb1052e36dc81dba40299b1556f1a39
expires: Mon, 01 May 2023 08:29:22 GMT
source-age: 194
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:24:22 GMT
via: 1.1 varnish
x-served-by: cache-lhr7385-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929463.714366,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 61c5dd6e65eb1b4ee65ea2d9fae6cce65d9ef779
expires: Mon, 01 May 2023 08:29:22 GMT
source-age: 194
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:24:22 GMT
via: 1.1 varnish
x-served-by: cache-lhr7374-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929463.714695,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 2d0d30a4cc89fcb71a88bff20f27a4c78eb36440
expires: Mon, 01 May 2023 08:29:22 GMT
source-age: 194
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:24:22 GMT
via: 1.1 varnish
x-served-by: cache-lhr7362-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929463.714570,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: f1339168dcf521d00ebafa09795b54c6fe653c4c
expires: Mon, 01 May 2023 08:29:22 GMT
source-age: 194
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:24:22 GMT
via: 1.1 varnish
x-served-by: cache-lhr7322-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929463.720385,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 1bbf3b4a69e4b5603d913c118a73e145a2cdf64b
expires: Mon, 01 May 2023 08:29:22 GMT
source-age: 194
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
date: Mon, 01 May 2023 08:24:22 GMT
via: 1.1 varnish
x-served-by: cache-lhr7353-LHR
x-cache: HIT
x-cache-hits: 25
x-timer: S1682929463.732916,VS0,VE0
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: b2c88c9777869d8b812a8fbe87cd7bfbef771c8a
expires: Mon, 01 May 2023 08:29:22 GMT
source-age: 194
content-range: bytes */3111
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:24:28 GMT
via: 1.1 varnish
x-served-by: cache-lhr7352-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929468.498552,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 0ac5378d1f1faa892d89bb9177fd3cd39d77c984
expires: Mon, 01 May 2023 08:29:28 GMT
source-age: 200
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:24:28 GMT
via: 1.1 varnish
x-served-by: cache-lhr7347-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929469.504574,VS0,VE2
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: d80d4c1b329f2437a28e52ba2c4377b42dd48e23
expires: Mon, 01 May 2023 08:29:28 GMT
source-age: 200
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:24:28 GMT
via: 1.1 varnish
x-served-by: cache-lhr7363-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929469.504692,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 87e645175867fd2b6f50b2f4f75271d6ea340168
expires: Mon, 01 May 2023 08:29:28 GMT
source-age: 200
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:24:28 GMT
via: 1.1 varnish
x-served-by: cache-lhr7335-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929469.511044,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: e57e569e5dd4e0ab3cacab34eb87a7d689bbe250
expires: Mon, 01 May 2023 08:29:28 GMT
source-age: 200
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:24:28 GMT
via: 1.1 varnish
x-served-by: cache-lhr7375-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929469.511240,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: b7be5facdba875efecd2b63b0a66677288ae97fb
expires: Mon, 01 May 2023 08:29:28 GMT
source-age: 200
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:24:28 GMT
via: 1.1 varnish
x-served-by: cache-lhr7364-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929469.506335,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: c5cadce6eda5c4f30efaf2031a3eebda0d872c55
expires: Mon, 01 May 2023 08:29:28 GMT
source-age: 200
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:24:28 GMT
via: 1.1 varnish
x-served-by: cache-lhr7341-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929469.517325,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 122bac67d31c90496db0539cb5130ac4ccda348e
expires: Mon, 01 May 2023 08:29:28 GMT
source-age: 200
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"75caf2abb861ff8763876c35a2dbf10080504ce29bcb99c503b0257ace8faac0"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 7C54:286F:1B7DDFB:1C92212:644F7673
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */3111
date: Mon, 01 May 2023 08:24:28 GMT
via: 1.1 varnish
x-served-by: cache-lhr7375-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929469.520948,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: fcdc010bff5e8458e7a6958d2ea9c0cd48a872be
expires: Mon, 01 May 2023 08:29:28 GMT
source-age: 200
content-length: 0

