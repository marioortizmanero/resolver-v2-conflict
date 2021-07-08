#!/usr/bin/env bash

find . -type f -name "Cargo.toml" -exec sed -i '
s/^# resolver = "2"/resolver = "2"/
t
s/resolver = "2"/# resolver = "2"/' {} \;
