# Year 17 Day 05

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y17/d05",
]
```

- [ ] Add shortcuts to the justfile:

```
y17d05-solve:
    cat ./y17/d05/input.txt | cargo run --release --package y17d05
y17d05-example:
    cat ./y17/d05/example.txt | cargo run --release --package y17d05
y17d05-test:
    cargo test --package y17d05 --lib -- --nocapture tests
y17d05-bench:
    cargo bench --package y17d05
```