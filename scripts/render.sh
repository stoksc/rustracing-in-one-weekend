#!/usr/bin/env bash

rm output/image.ppm

cargo run --release && \
open output/image.ppm
