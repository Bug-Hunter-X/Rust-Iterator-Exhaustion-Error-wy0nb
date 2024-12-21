# Rust Iterator Exhaustion Bug

This repository demonstrates a common error in Rust when working with iterators: failing to properly handle the end of the iterator. The `bug.rs` file contains code that attempts to access elements beyond the iterator's end, resulting in a panic. The solution, provided in `bugSolution.rs`, shows how to safely check for the end of the iterator using pattern matching.