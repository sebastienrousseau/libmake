#!/usr/bin/env bash

set -e

if [ "$(git symbolic-ref --short HEAD)" != "main" ]; then
    echo "Can only release from main!" >&2
    exit 1
fi

RELEASE_LEVEL=$1

case "$RELEASE_LEVEL" in
    minor|major|patch)
        echo "Creating a $RELEASE_LEVEL release..."
        ;;
    *)
        echo "Release level should be minor/major/patch" >&2
        exit 1
        ;;
esac

cargo release --execute --sign --no-publish --no-dev-version $RELEASE_LEVEL
