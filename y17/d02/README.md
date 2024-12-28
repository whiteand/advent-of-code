# Year 17 Day 02

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y17/d02",
]
```

- [ ] Add shortcuts to the justfile:

```
y17d02-solve:
    cat ./y17/d02/input.txt | cargo run --release --package y17d02
y17d02-example:
    cat ./y17/d02/example.txt | cargo run --release --package y17d02
y17d02-test:
    cargo test --package y17d02 --lib -- --nocapture tests
y17d02-bench:
    cargo bench --package y17d02
```