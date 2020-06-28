# Alo
Alo (short for aluminum oxide) is a proof of concept mutation based fuzzer that fuzzes python's urllib3 from rust.

# Building
To build Alo you need to first install a nightly version of rust.
After that you can clone the repository and run `cargo build` like normal.

# Running fuzztest
To start fuzzing run `alo <number of tests per thread> <threads>`.
This means that if you run `alo 100 2` it will run 200 test cases.
