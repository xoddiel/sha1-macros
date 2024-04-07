# SHA1 macros
The [`sha1-macros`](https://crates.io/crates/sha1-macros) crate allows you to compute SHA1 hashes at compile-time. 

```rust
assert_eq!(sha1_hex!("this is a test"), "fa26be19de6bff93f70bc2308434e4a440bbad02");
assert_eq!(sha1_bytes!("this is a test"), hex!("fa26be19de6bff93f70bc2308434e4a440bbad02"));
```

## Why macros and not `const fn`?
Simple answer: It is not yet possible to create a `&'static str` at compile-time using `const fn`. By providing macros,
we remove the need to encode your hash digest into hex or base64 at runtime. Note that this has the limitation that the
input of `sha1_*` macros must be either a string (`"value"`) or a byte (`b"value"`) literal. **It cannot be a `const`
value.**
