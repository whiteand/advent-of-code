# Dependency upgrades

Checklist from the library review on 2026-11-05. Versions below are the latest crates.io releases from that day.

Crates that are already the usual choice and current (`glam`, `itertools`, `rayon`, `regex`, `thiserror`, `derive_more`, `rstest`, `smallvec`, `tracing`, `tracing-subscriber`, `good_lp`, `rustsat`, `libfuzzer-sys`, `hex`) are not listed.

## Replace

- [x] **Replace `fxhash` with `rustc-hash`.** `rustc-hash` 2.1.3 is a workspace dependency. Call sites use `rustc_hash::FxHashMap` / `FxHashSet`. Unused `fxhash` pins were removed from `y24/d22`, `y25/d07`, `y25/d08`, and `y25/d09`.

- [x] **Use `md-5` everywhere instead of `md5`.** `md-5` 0.11.0 is a workspace dependency. `y15/d04`, `y16/d14`, and `y16/d17` hash through `Md5` and the `Digest` trait.

- [ ] **Drop `array-init`.** `2.1.0` (December 2022) is the last release, and the only call is `array_init::from_iter` in `advent_utils/src/math/systems.rs`. Build the `[&mut [Rat]; EQS]` with `[T; N]::each_mut()` instead, then remove the dependency from `advent_utils/Cargo.toml`.

- [ ] **Replace `fixed-slice-vec` with `arrayvec` when `vec_on_stack!` is next touched.** `fixed-slice-vec` `0.10.0` (August 2022) backs that macro, which is used only in `y24/d15/src/lib.rs`. `arrayvec` is the usual stack-allocated vec. Leave it until that code changes.

## Bump

- [ ] **Bump `rand` off `0.8.5`.** [RUSTSEC-2026-0097](https://rustsec.org/advisories/RUSTSEC-2026-0097) affects `0.8.5` (`thread_rng` is unsound if a custom logger also draws from it). The `0.8` fix is `>= 0.8.6`. Current is `0.10.3`. `advent_utils` re-exports `rand`, and real uses are `thread_rng()` in `y16/d21`, `SmallRng` in `y24/d17`, and `rand::distributions::Slice` in `advent_utils/src/genetic.rs`. On `0.10`, `thread_rng()` is `rng()` and `rand::distributions` is `rand::distr`. Delete the unused `rand = "0.8.4"` from `y23/d24`.

- [ ] **Put every `petgraph` dependency on `0.8`.** `0.8.3` is current and already used by `y22/d21` and `y25/d11`. Still on `0.6`: `y17/d07`, `y19/d06`, `y23/d25` (`0.6.4`), `y24/d23`, `y24/d24`.

- [ ] **Bump `pathfinding` to `4.16`.** Latest is `4.16.0` (September 2026). In use at `4.12.0`: `y16/d13`, `y16/d17`, `y16/d24`, `y24/d16`, `y24/d18`, `y24/d20`. In use at `4.6.0`: `y23/d17`. Declared and unused, so delete instead of bumping: `y16/d11`, `y16/d22`, `y23/d23`, `y24/d23`.

## Remove unused dependencies

- [ ] **`graphalgs` `0.1.1` in `y23/d25`.** Never imported. Removing it also unblocks that day's `petgraph` bump. The crate itself is barely used upstream (about 400 downloads in 90 days).
- [ ] **`edit-distance` `2.1.3` in `y15/d19`.** Never imported. The maintained, widely used string-distance crate is `strsim`, if a later solution needs it.
- [ ] **`disjoint` `0.8.0` in `y24/d23`.** Never imported. Union-find in this repo lives in `advent_utils::disjoint_set` (`y25/d08` uses it).

## Later

- [ ] **New parsers go in `winnow`, not `nom`.** `nom` `8.0.0` (January 2025) still works and `advent_utils` re-exports it. `winnow` now has the higher download count and the newer releases (`1.0.4`, July 2026). Rewriting the existing re-export is a separate migration, not part of a version bump.

- [ ] **Revisit `mockalloc` `0.1.2` if the grid allocation tests break.** Last release is November 2020. It is a dev-dependency of `advent_utils`, used only by `record_allocs` in `advent_utils/src/grid.rs`. There is no drop-in replacement.

## Leave as-is

- **`criterion` `0.5.1` and `divan`.** Both are maintained. Criterion's current release is `0.8.2` and it is still the more widely used harness. Newer days already benchmark with `divan`. Keeping both is a style split, not a stale dependency.
- **`hex` `0.4.3`.** No release since 2021 because the API is finished. It is still the standard crate.
- **`tracing-chrome` `0.7.2`.** Last release March 2024, still the crate for Chrome trace output, no advisory.
