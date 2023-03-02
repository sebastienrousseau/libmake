#!/usr/bin/env bash

# TAG=$(curl -s https://api.github.com/repos/sebastienrousseau/libmake/releases/latest | grep tag_name | grep -Eo "v[0-9]+\.[0-9]+\.[0-9]+")
TAG=v0.1.1

get_sha() {
  local DOWNLOAD_PATH="https://github.com/sebastienrousseau/libmake/releases/download/$TAG/libmake-$TAG-$1.tar.gz.sha256"
  curl -s -L $DOWNLOAD_PATH | cut -d ' ' -f 1
}

read -r -d '' PKGBUILD <<EOF
# Maintainer: Sebastian Rousseau <sebastian.rousseau [@] gmail [dot] com>

pkgname=libmake
pkgver=${TAG:1}
pkgrel=1
pkgdesc='A code generator to reduce repetitive tasks and build high-quality Rust libraries'
arch=('x86_64' 'aarch64' 'armv7')
url='https://github.com/sebastienrousseau/libmake'
license=('GPL-3.0-or-later')
depends=()
provides=("\${pkgname%-bin}")
conflicts=("\${pkgname%-bin}")

source_x86_64=("\${url}/releases/download/v\${pkgver}/\${pkgname%-bin}-v\${pkgver}-x86_64-unknown-linux-musl.tar.gz")
source_aarch64=("\${url}/releases/download/v\${pkgver}/\${pkgname%-bin}-v\${pkgver}-aarch64-unknown-linux-gnu.tar.gz")
source_armv7=("\${url}/releases/download/v\${pkgver}/\${pkgname%-bin}-v\${pkgver}-armv7-unknown-linux-gnueabihf.tar.gz")

sha256sums_x86_64=('$(get_sha x86_64-unknown-linux-musl)')
sha256sums_aarch64=('$(get_sha aarch64-unknown-linux-gnu)')
sha256sums_armv7=('$(get_sha armv7-unknown-linux-gnueabihf)')

package() {
  install -Dm755 "\${srcdir}/libmake" "\${pkgdir}/usr/bin/libmake"
}
EOF

echo "$PKGBUILD"
