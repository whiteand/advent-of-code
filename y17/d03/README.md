# Year 17 Day 03

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y17/d03",
]
```

- [ ] Add shortcuts to the justfile:

```
y17d03-solve:
    cat ./y17/d03/input.txt | cargo run --release --package y17d03
y17d03-example:
    cat ./y17/d03/example.txt | cargo run --release --package y17d03
y17d03-test:
    cargo test --package y17d03 --lib -- --nocapture tests
y17d03-bench:
    cargo bench --package y17d03
```