#!/bin/bash

rm -rf ./target/doc
RUSTDOCFLAGS="--enable-index-page -Zunstable-options --html-in-header ./source/pagefind.html --cfg docsrs" cargo +nightly doc --no-deps --all-features
cd ./target/doc
pagefind --site ./
cp -a ../../source/loadpagefind.js ./