# spain-vat-id
![Build](https://github.com/jsgm/spain-vat-id/actions/workflows/rust.yml/badge.svg)
[![Crate](https://img.shields.io/crates/v/spain-vat-id.svg)](https://crates.io/crates/spain-vat-id)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.59+-lightgray.svg)
![Last Commit](https://img.shields.io/github/last-commit/jsgm/spain-vat-id)

A dead simple crate for validating/verifying Spanish VAT ID numbers written in [Rust](https://rust-lang.org).

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
check_nif_digit(val: u32) -> char
is_valid_nif(v: &str) -> Result<(), Error>
is_valid_nie(v: &str) -> Result<(), Error>
```

```rust
// NIF
match is_valid_nif("9874`457T") {
    Ok(()) => println!("Valid!"),
    Err(e) => println!("{}", e),
}

// NIE
match is_valid_nie("Y5937943R") {
    Ok(()) => println!("Valid!"),
    Err(e) => println!("{}", e),
}

// Get check digit
let digit = check_nif_digit(24591177); // 'Z'
```

## References
- [Cálculo del dígito de control del NIF/NIE](https://www.interior.gob.es/opencms/es/servicios-al-ciudadano/tramites-y-gestiones/dni/calculo-del-digito-de-control-del-nif-nie/)
- https://en.wikipedia.org/wiki/VAT_identification_number
- https://es.wikipedia.org/wiki/N%C3%BAmero_de_identificaci%C3%B3n_fiscal
- https://es.wikipedia.org/wiki/N%C3%BAmero_de_identidad_de_extranjero
