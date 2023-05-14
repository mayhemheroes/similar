#![no_main]
use libfuzzer_sys::fuzz_target;
use similar::{Algorithm, capture_diff_slices};

fuzz_target!(|input: (&[u8], &[u8])| {
    let (a, b) = input;
    capture_diff_slices(Algorithm::Myers, a, b);
});