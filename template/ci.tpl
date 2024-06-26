name: ❯ {name} release

on:
  pull_request:
    branches:
      - main
      - feat/{name}
  push:
    branches:
      - main
      - feat/{name}

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
        uses: actions/checkout@v4

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
        uses: actions/checkout@v4

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
        uses: actions/checkout@v4

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
          curl -sL https://github.com/mozilla/grcov/releases/download/v0.8.19/grcov-x86_64-unknown-linux-gnu.tar.bz2 | tar jxf - -C "${HOME}/.local/bin"
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
        uses: actions/checkout@v4
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
        uses: actions/checkout@v4

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
        uses: actions/checkout@v4

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
            curl -sSL -H "Authorization: token ${GITHUB_TOKEN}" -H "Accept: application/vnd.github.v3+json" -L "${GITHUB_API_URL}/repos/${GITHUB_REPOSITORY}/actions/runs/${RUN_ID}/artifacts/{name}" -o "target/package/{name}"
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
          release_name: {name}} 🦀 v${{ env.VERSION }}
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
        uses: actions/checkout@v4

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
