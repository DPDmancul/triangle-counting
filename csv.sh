#!/bin/bash

while true; do cargo run --release | sed -z 's/\n/, /' >> 1_000_000.csv; done
