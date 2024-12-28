# Year 17 Day 06

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y17/d06",
]
```

- [ ] Add shortcuts to the justfile:

```
y17d06-solve:
    cat ./y17/d06/input.txt | cargo run --release --package y17d06
y17d06-example:
    cat ./y17/d06/example.txt | cargo run --release --package y17d06
y17d06-test:
    cargo test --package y17d06 --lib -- --nocapture tests
y17d06-bench:
    cargo bench --package y17d06
```