#!/usr/bin/env bash

set -e

cargo +nightly doc --document-private-items --features doc

cat > target/doc/index.html <<HTML
<meta http-equiv="refresh" content="0;url=sorting_visualization/index.html">
HTML
