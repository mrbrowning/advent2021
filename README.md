Advent of Code 2021
===================

An attempt to write maximally functional-style Rust, because it's fun but not as applicable in the domains I work in!

Obviously I wouldn't normally bulldoze through `Result`s and `Option`s via `unwrap()` with such reckless abandon, but since the input is known, fixed, and a panic is basically as informative as an error at this scale, I opted to avoid what would just amount to boilerplate in this context.
