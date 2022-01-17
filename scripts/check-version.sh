#! /bin/bash

# exit when any command fails
set -e

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

function get_current_version() {
  CURRENT_VERSION=$(tomlq package.version -f ./node/Cargo.toml | sed 's/"//g')

  branch_name="$(git symbolic-ref HEAD 2>/dev/null)" ||
  branch_name="(unnamed branch)"     # detached HEAD (or possibly github workflow?)
  branch_name=${branch_name##refs/heads/}

  if [ $GITHUB_REF != "" ]; then
    branch_name=${GITHUB_REF##*/}
  fi
  
  if [ "$branch_name" != "main" ]; then
    CURRENT_VERSION=$(printf '%s-%s' "$CURRENT_VERSION" "$branch_name")
    IS_PRERELEASE=true;
  fi

  release_type="release"
  if [ $IS_PRERELEASE ]; then
    release_type="prerelease"
  fi
  
  printf "Current %s found to be %s\n" "$release_type" "$CURRENT_VERSION"
}

# Get published git tags that match semver regex with a "v" prefixbash then remove the "v" character
PUBLISHED_VERSIONS=$(git tag | grep "^v[0-9]\+\.[0-9]\+\.[0-9]\+\(\-[a-zA-Z-]\+\(\.[0-9]\+\)*\)\{0,1\}$" | sed 's/^v\(.*\)$/\1/')
# Get the current version from node Cargo.toml

get_current_version

if check_version_greater "$CURRENT_VERSION" "$PUBLISHED_VERSIONS"; then
  echo "##[set-output name=VERSION;]v$CURRENT_VERSION"
  echo "##[set-output name=BUILD_DATE;]$(date -u +'%Y-%m-%dT%H:%M:%SZ')"
  echo "##[set-output name=IS_NEW_VERSION;]true"
  if [ $IS_PRERELEASE ]; then
    echo "##[set-output name=IS_PRERELEASE;]true"
  else
    echo "##[set-output name=IS_PRERELEASE;]false"
  fi
else
  echo "##[set-output name=IS_NEW_VERSION;]false"
fi
