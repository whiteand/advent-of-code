# Year 17 Day 04

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y17/d04",
]
```

- [ ] Add shortcuts to the justfile:

```
y17d04-solve:
    cat ./y17/d04/input.txt | cargo run --release --package y17d04
y17d04-example:
    cat ./y17/d04/example.txt | cargo run --release --package y17d04
y17d04-test:
    cargo test --package y17d04 --lib -- --nocapture tests
y17d04-bench:
    cargo bench --package y17d04
```