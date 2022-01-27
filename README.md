# Inertia

Inertia is a computational mathematics library for Rust.

## Features

This is a checklist of the main intended features and their current implementation progress.
Features marked with an asterisk have their basic implementation done but need work on
additional functions, arithmetic, conversion, etc.

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

## Usage

To use Inertia in a Rust crate add it to your crate dependencies.
```
[dependencies]
inertia = "0.1"
```

Inertia provides a `prelude` module for easy importing. Use
```
use inertia::prelude::*;
```
to make all of the primary features of Inertia available in the top level scope.

### REPL/Jupyter notebook

It is also possible to use Inertia in a read-eval-print-loop (REPL) or Jupyter notebook for more convenient prototyping or experimentation. 
Install the [Evcxr](https://github.com/google/evcxr) REPL or Jupyter notebook, open a new REPL/notebook, and add Inertia as a dependency with 
```
dep: inertia = "0.1"
```
Then import the `prelude` module with `use inertia::prelude::*;`.

