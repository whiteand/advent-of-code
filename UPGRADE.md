# Dependency upgrades

Checklist from the library review on 2026-11-05. Versions below are the latest crates.io releases from that day.

Crates that are already the usual choice and current (`glam`, `itertools`, `rayon`, `regex`, `thiserror`, `derive_more`, `rstest`, `smallvec`, `tracing`, `tracing-subscriber`, `good_lp`, `rustsat`, `libfuzzer-sys`, `hex`) are not listed.

## Replace

- [x] **Replace `fxhash` with `rustc-hash`.** `rustc-hash` 2.1.3 is a workspace dependency. Call sites use `rustc_hash::FxHashMap` / `FxHashSet`. Unused `fxhash` pins were removed from `y24/d22`, `y25/d07`, `y25/d08`, and `y25/d09`.

- [x] **Use `md-5` everywhere instead of `md5`.** `md-5` 0.11.0 is a workspace dependency. `y15/d04`, `y16/d14`, and `y16/d17` hash through `Md5` and the `Digest` trait.

- [x] **Drop `array-init`.** `solve_system` builds `[&mut [Rat]; EQS]` with `[T; N]::each_mut()`.

- [x] **Replace `fixed-slice-vec` with `arrayvec`.** `y24/d15` stores the horizontal box chain in `ArrayVec<usize, 5>`. The `vec_on_stack!` macro is gone.

## Bump

- [x] **Bump `rand` to `0.10.3`.** `thread_rng()` is `rng()`, `gen_range` is `random_range`, and `distributions::Slice` is `distr::slice::Choose`. The unused `rand` pin was removed from `y23/d24`. The `small_rng` feature no longer exists; `SmallRng` is always available.

- [x] **Put every `petgraph` dependency on `0.8.3`.** It is a workspace dependency. `y19/d06` now builds `NodeIndex<usize>` explicitly for Dijkstra. Unused `graphalgs`, which was keeping `petgraph` 0.6 in `y23/d25`, was removed.

- [x] **Bump `pathfinding` to `4.16.0`.** It is a workspace dependency. Unused pins were removed from `y16/d11`, `y16/d22`, `y23/d23`, and `y24/d23`.

## Remove unused dependencies

- [x] **`graphalgs` `0.1.1` in `y23/d25`.** Never imported, and it depended on `petgraph` 0.6. Removed with the `petgraph` 0.8 bump.
- [ ] **`edit-distance` `2.1.3` in `y15/d19`.** Never imported. The maintained, widely used string-distance crate is `strsim`, if a later solution needs it.
- [ ] **`disjoint` `0.8.0` in `y24/d23`.** Never imported. Union-find in this repo lives in `advent_utils::disjoint_set` (`y25/d08` uses it).

## Later

- [ ] **New parsers go in `winnow`, not `nom`.** `nom` `8.0.0` (January 2025) still works and `advent_utils` re-exports it. `winnow` now has the higher download count and the newer releases (`1.0.4`, July 2026). Rewriting the existing re-export is a separate migration, not part of a version bump.

- [ ] **Revisit `mockalloc` `0.1.2` if the grid allocation tests break.** Last release is November 2020. It is a dev-dependency of `advent_utils`, used only by `record_allocs` in `advent_utils/src/grid.rs`. There is no drop-in replacement.

## Leave as-is

- **`criterion` `0.5.1` and `divan`.** Both are maintained. Criterion's current release is `0.8.2` and it is still the more widely used harness. Newer days already benchmark with `divan`. Keeping both is a style split, not a stale dependency.
- **`hex` `0.4.3`.** No release since 2021 because the API is finished. It is still the standard crate.
- **`tracing-chrome` `0.7.2`.** Last release March 2024, still the crate for Chrome trace output, no advisory.
