# Year 17 Day 01

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y17/d01",
]
```

- [ ] Add shortcuts to the justfile:

```
y17d01-solve:
    cat ./y17/d01/input.txt | cargo run --release --package y17d01
y17d01-example:
    cat ./y17/d01/example.txt | cargo run --release --package y17d01
y17d01-test:
    cargo test --package y17d01 --lib -- --nocapture tests
y17d01-bench:
    cargo bench --package y17d01
```