# ec2b

```toml
# Cargo.toml
[dependencies]
ec2b = { git = "https://github.com/juliuskreutz/ec2b-rs" }
```

```rust
let seed = std::fs::read("ec2b_seed.bin").unwrap();

let key = ec2b::derive(&seed);
```
