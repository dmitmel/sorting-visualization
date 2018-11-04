#!/usr/bin/env bash

set -e

if [[ "$TRAVIS" != true ]]; then
  echo "This script is intended to run only on Travis CI!"
  exit 1
fi

if [[ "$TRAVIS_BRANCH" != master ]]; then
  echo "Skipping docs because this branch is not master"
  exit 0
fi

if [[ "$TRAVIS_PULL_REQUEST" != false ]]; then
  echo "Skipping docs because this is a pull request"
  exit 0
fi

if [[ "$TRAVIS_RUST_VERSION" != nightly ]]; then
  echo "Skipping docs because they can built only on nightly"
  exit 0
fi

./scripts/docs.sh

sudo pip install ghp-import
ghp-import -n target/doc
git push -fq "https://${GH_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git" gh-pages
