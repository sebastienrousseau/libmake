name: 🧪 Document

on:
  push:
    branches:
      - main
  pull_request:
    branches:
      - main
  release:
    types: [created]

jobs:
  all:
    name: Document
    if: github.ref == 'refs/heads/main' && github.event_name == 'push'
    runs-on: ubuntu-latest
    concurrency:
      group: ${{ github.workflow }}-${{ github.ref }}
    steps:
      - uses: hecrj/setup-rust-action@v2
        with:
          rust-version: nightly

      - uses: actions/checkout@v4

      - name: Update libssl
        run: |
          sudo apt-get update
          sudo apt-get install -y libssl1.1

      - name: Generate documentation for all features and publish it
        run: |
          RUSTDOCFLAGS="--cfg docsrs" \
            cargo doc --no-deps --all-features --workspace
            # Write index.html with redirect
            echo '<html><head><meta http-equiv="refresh" content="0; url=/{name}/"></head><body></body></html>' > ./target/doc/index.html

      - name: Deploy
        uses: actions/upload-artifact@v4
        with:
          name: documentation
          path: target/doc
          if-no-files-found: error
          retention-days: 1

      - name: Write CNAME file
        run: echo '{name}.github.io' > ./target/doc/CNAME

      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v4
        with:
          cname: true
          commit_message: Deploy documentation at ${{ github.sha }}
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_branch: gh-pages
          publish_dir: ./target/doc
          user_email: actions@users.noreply.github.com
          user_name: github-actions
