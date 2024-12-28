# Year 17 Day 07

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y17/d07",
]
```

- [ ] Add shortcuts to the justfile:

```
y17d07-solve:
    cat ./y17/d07/input.txt | cargo run --release --package y17d07
y17d07-example:
    cat ./y17/d07/example.txt | cargo run --release --package y17d07
y17d07-test:
    cargo test --package y17d07 --lib -- --nocapture tests
y17d07-bench:
    cargo bench --package y17d07
```