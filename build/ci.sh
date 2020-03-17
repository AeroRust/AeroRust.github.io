#!/bin/bash
# Original source: https://github.com/rust-lang/book/blob/master/ci/build.sh
set -e

export PATH=$PATH:/home/travis/.cargo/bin;

echo 'Testing'
mdbook test

echo 'Building'
mdbook build
