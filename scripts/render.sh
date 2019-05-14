#!/usr/bin/env bash

cargo run --release > output/image.ppm
open output/image.ppm
