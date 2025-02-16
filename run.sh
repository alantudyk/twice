#!/bin/bash

rustc -C opt-level=3 e.rs -o e &&
time ./e enwik9 /tmp/enwik9.twice &&
rustc -C opt-level=3 d.rs -o d &&
time ./d /tmp/enwik9.twice /tmp/enwik9.dec &&
cmp enwik9 /tmp/enwik9.dec &&
echo &&
ls -l /tmp/enwik9.twice &&
echo
