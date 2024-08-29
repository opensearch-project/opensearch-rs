#!/usr/bin/env bash

set -ex

IFS='.' read -r -a VERSION_COMPONENTS <<< "$1"
MAJOR="${VERSION_COMPONENTS[0]}"
MINOR="${VERSION_COMPONENTS[1]}"
PATCH="${VERSION_COMPONENTS[2]}"

if [[ -z "$MAJOR" || -z "$MINOR" || -z "$PATCH" ]]; then
  echo "Usage: $0 <major>.<minor>.<patch>"
  exit 1
fi

VERSION="$MAJOR.$MINOR.$PATCH"

CURRENT_VERSION=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name == "opensearch") | .version')

cargo install cargo-edit \
  && cargo set-version "${VERSION}"

s=$(command -v gsed || command -v sed)

"$s" -i'' -E "s/\/\/\! opensearch =( \{ version =)? \"${CURRENT_VERSION}\"/\/\/\! opensearch =\1 \"${VERSION}\"/" opensearch/src/lib.rs
