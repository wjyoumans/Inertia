# Inertia

Inertia is a computational mathematics library for Rust.

## Dependencies

Inertia relies heavily on the C libraries [Flint](https://flintlib.org/doc/),
[Arb](https://arblib.org/), and [Antic](https://github.com/wbhart/antic). See the FFI crates
[flint-sys](https://crates.io/crates/flint-sys), [arb-sys](https://crates.io/crates/arb-sys), and 
[antic-sys](https://crates.io/crates/antic-sys) for details on installation.

## Features

This is a checklist of the main intended features and their current implementation progress.
Features marked with an asterisk have their basic implementation done but need work on
additional functions, arithmetic, conversion, etc.

If there is a feature that is missing that would be beneficial to you don't hesitate to open an issue.

- [x] arbitrary precision integers
- [x] rational numbers
- [ ] \*real numbers
- [ ] \*complex numbers
- [ ] \*integers mod n
- [ ] \*finite fields
- [ ] \*p-adic/q-adic numbers
- [ ] polynomials
    - [x] integer polynomials
    - [x] rational polynomials
    - [ ] real polynomials
    - [ ] complex polynomials 
    - [ ] \*polynomials over integers mod n
    - [ ] \*polynomials over finite fields
    - [ ] polynomials over p-adic/q-adics
- [ ] matrices
    - [x] integer matrices
    - [x] rational matrices
    - [ ] real matrices
    - [ ] complex matrices 
    - [ ] \*matrices over integers mod n
    - [ ] \*matrices over finite fields
    - [ ] matrices over p-adic/q-adics
- [ ] multivariate polynomials
- [ ] rational functions (currently disabled)
- [ ] \*number fields
