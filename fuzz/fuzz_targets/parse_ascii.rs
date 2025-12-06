#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|s: &str| {
    let grid = advent_utils::parse::ascii_grid(s);
    assert_eq!(grid.rows_len(), s.lines().count());
    for (i, line) in s.lines().enumerate() {
        assert_eq!(grid.cols(i), line.len());
    }
});
