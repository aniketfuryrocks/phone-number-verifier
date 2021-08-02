![Crates.io](https://img.shields.io/crates/d/phone-number-verifier?style=flat-square)
![docs.rs](https://img.shields.io/docsrs/phone-number-verifier?style=flat-square)

# Phone Number Verifier
*phone number verification library for rust*

### With Country Code

```rust
pub fn verify_phone_number_with_country_code(ph: &str) -> bool
```

```regex
^(\+\d{1,2}\s?)?1?\-?\.?\s?\(?\d{3}\)?[\s.-]?\d{3}[\s.-]?\d{4}$
```

### Without Country Code

```rust
pub fn verify_phone_number_without_country_code(ph: &str) -> bool
```

```regex
^1?\-?\.?\s?\(?\d{3}\)?[\s.-]?\d{3}[\s.-]?\d{4}$
```

