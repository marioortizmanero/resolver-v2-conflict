#!/usr/bin/env bash

find . -type f -exec sed -i "s/foo/bar/g" {} \;
