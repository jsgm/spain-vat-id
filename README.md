# spain-vat-id
[![Crate](https://img.shields.io/crates/v/spain-vat-id.svg)](https://crates.io/crates/spain-vat-id)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.59+-lightgray.svg)](https://github.com/rust-random/rand#rust-version-requirements)

A dead simple crate for validating/verifying Spanish VAT ID numbers written in [Rust](rust-lang.org).

This crate allows you to easily check and verify the following ID numbers:
- Documento Nacional de Identidad (DNI)
- Número de Identificación Fiscal (NIF)
- Número de Identificación de Extranjero (NIE)

## Installation
```
cargo add spain-vat-id
```

## Functions
```rust
nif_check_digit(val: u32) -> char
is_valid_nie(v: &str) -> (bool, String)
is_valid_nif(v: &str) -> (bool, String)
```

```rust
// NIF checking
let nif = "9874`457T";
let (valid, explain) = is_valid_nif(nif);
if !valid{
    println!("{}", explain);
    // Not valid: Char '`' at position 5 is not a number
}
```

## References
- https://en.wikipedia.org/wiki/VAT_identification_number
- https://es.wikipedia.org/wiki/N%C3%BAmero_de_identificaci%C3%B3n_fiscal
- https://es.wikipedia.org/wiki/N%C3%BAmero_de_identidad_de_extranjero
- http://www.interior.gob.es/web/servicios-al-ciudadano/dni/calculo-del-digito-de-control-del-nif-nie

