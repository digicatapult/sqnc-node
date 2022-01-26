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

# Get published chart version from github-pages deployment
REPOSITORY_NAME=$(echo "$GITHUB_REPOSITORY" | awk -F / '{print $2}')
PUBLISHED_VERSIONS=$(curl -s https://$GITHUB_REPOSITORY_OWNER.github.io/$REPOSITORY_NAME/index.yaml | \
yq eval '.entries.[env(REPOSITORY_NAME)] | .[0].version' -)
# Get the current version from helm chart
CURRENT_VERSION=$(yq eval '.version' ./helm/vitalam-node/Chart.yaml)

if check_version_greater "$CURRENT_VERSION" "$PUBLISHED_VERSIONS"; then
  echo "##[set-output name=VERSION;]$CURRENT_VERSION"
  echo "##[set-output name=BUILD_DATE;]$(date -u +'%Y-%m-%dT%H:%M:%SZ')"
  echo "##[set-output name=IS_NEW_VERSION;]true"
else
  echo "##[set-output name=IS_NEW_VERSION;]false"
fi
