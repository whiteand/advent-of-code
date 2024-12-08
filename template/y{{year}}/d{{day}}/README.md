# Year {{year}} Day {{day}}

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y{{year}}/d{{day}}",
]
```

- [ ] Add shortcuts to the justfile:

```
y{{year}}d{{day}}-solve:
    cat ./y{{year}}/d{{day}}/input.txt | cargo run --release --package y{{year}}d{{day}}
y{{year}}d{{day}}-example:
    cat ./y{{year}}/d{{day}}/example.txt | cargo run --release --package y{{year}}d{{day}}
y{{year}}d{{day}}-test:
    cargo test --package y{{year}}d{{day}} --lib -- --nocapture tests
y{{year}}d{{day}}-bench:
    cargo bench --package y{{year}}d{{day}}
```