#! /bin/bash

# exit when any command fails
set -e

function check_versions_consistent () {
  local PACKAGE_VERSION=$(yq eval '.version' ./vitalam-node/package.json)
  local PACKAGE_LOCK_VERSION=$(yq eval '.version' ./vitalam-node/package-lock.json)
  local APP_VERSION=$(tomlq .package.version ./runtime/Cargo.toml | sed 's/"//g')

  if [ "$PACKAGE_VERSION" != "$PACKAGE_LOCK_VERSION" ] ||
    [ "$PACKAGE_VERSION" != "$APP_VERSION" ]; then
    echo "Inconsistent versions detected"
    echo "PACKAGE_VERSION: $PACKAGE_VERSION"
    echo "PACKAGE_LOCK_VERSION: $PACKAGE_LOCK_VERSION"
    echo "APP_VERSION: $APP_VERSION"
    exit 1
  fi
}

check_versions_consistent

function check_version_greater () {
  local current=$1
  local git_versions="${2:-0.0.0}"

  # check if current exists in git_versions, if so not a new version
  if [ -n "$(printf "$git_versions" | grep -Fx $current)" ]; then
    return 1
  fi

  # sort all - note crazy hack to deal with prerelease versions by appending a _ character to release versions
  local sorted_versions=($(printf "$git_versions\n$current" | awk '{ if ($1 ~ /-/) print; else print $0"_" ; }' | sort -rV | sed 's/_$//'))

  # check if the top sorted version equals the current verison. If so we have a new version
  if [ "${sorted_versions[0]}" == "$current" ]; then
    return 0
  else
    return 1
  fi
}

# Get published git tags that match semver regex with a "v" prefix then remove the "v" character
PUBLISHED_VERSIONS=$(git tag | grep "^v[0-9]\+\.[0-9]\+\.[0-9]\+\(\-[a-zA-Z-]\+\(\.[0-9]\+\)*\)\{0,1\}$" | sed 's/^v\(.*\)$/\1/')
# Get the current version from node Cargo.toml
CURRENT_VERSION=$(yq eval '.version' ./vitalam-node/package.json)

if check_version_greater "$CURRENT_VERSION" "$PUBLISHED_VERSIONS"; then
  echo "##[set-output name=VERSION;]v$CURRENT_VERSION"
  echo "##[set-output name=BUILD_DATE;]$(date -u +'%Y-%m-%dT%H:%M:%SZ')"
  echo "##[set-output name=IS_NEW_VERSION;]true"
  if [[ $CURRENT_VERSION =~ [-] ]]; then
    echo "##[set-output name=IS_PRERELEASE;]true"
    echo "##[set-output name=NPM_RELEASE_TAG;]next"
  else
    echo "##[set-output name=IS_PRERELEASE;]false"
    echo "##[set-output name=NPM_RELEASE_TAG;]latest"
  fi
else
  echo "##[set-output name=IS_NEW_VERSION;]false"
fi
