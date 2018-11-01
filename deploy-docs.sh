#!/usr/bin/env bash

set -e

if [[ "$TRAVIS" == true ]]; then
  if [[ "$TRAVIS_BRANCH" == master ]]; then
    if [[ "$TRAVIS_PULL_REQUEST" == false ]]; then
      rustup toolchain install nightly # nightly version is requierd for building docs
      cargo +nightly doc --document-private-items

      CRATE_NAME="$(echo "$TRAVIS_REPO_SLUG" | cut -d '/' -f 2 | tr '-' '_')"
      cat > target/doc/index.html <<HTML
<meta http-equiv="refresh" content="0;url=$CRATE_NAME/index.html">
HTML

      sudo pip install ghp-import
      ghp-import -n target/doc
      git push -fq "https://${GH_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git" gh-pages
    else
      echo "Skipping docs because this is a pull request"
    fi
  else
    echo "Skipping docs because this branch is not master"
  fi

  exit 0
else
  echo "This script is intended to run only on Travis CI!"
  exit 1
fi
