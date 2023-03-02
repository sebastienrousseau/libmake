#!/usr/bin/env bash

ARCH=$(uname -sm)
BIN_DIR=${1:-/usr/local/bin}

case $ARCH in
"Linux x86_64")
    TARGET="x86_64-unknown-linux-musl"
    ;;
"Linux aarch64")
    TARGET="aarch64-unknown-linux-gnu"
    ;;
"Linux armv7l")
    TARGET="armv7-unknown-linux-gnueabihf"
    ;;
"Darwin x86_64")
    TARGET="x86_64-apple-darwin"
    ;;
esac

if [ -z "$TARGET" ]; then
    echo "Unknown target architecture: $ARCH" >&2
    exit 1
fi

if [ ! -d "$BIN_DIR" ]; then
    echo "Can't find the target install directory: $BIN_DIR" >&2
    exit 1
fi

LIBMAKE_VERSION=$(curl -s https://api.github.com/repos/sebastienrousseau/libmake/releases/latest | grep tag_name | grep -Eo "v[0-9]+\.[0-9]+\.[0-9]+")
DOWNLOAD_PATH="https://github.com/sebastienrousseau/libmake/releases/download/$LIBMAKE_VERSION/libmake-$LIBMAKE_VERSION-$TARGET.tar.gz"

curl -s -L $DOWNLOAD_PATH | tar -xz -C $BIN_DIR -f -
