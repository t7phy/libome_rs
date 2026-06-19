# libome-rs

[![CI](https://github.com/t7phy/libome_rs/actions/workflows/ci.yml/badge.svg)](https://github.com/t7phy/libome_rs/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/libome-rs.svg)](https://crates.io/crates/libome-rs)
[![docs.rs](https://docs.rs/libome-rs/badge.svg)](https://docs.rs/libome-rs)
[![License: GPL-3.0-or-later](https://img.shields.io/crates/l/libome-rs.svg)](LICENSE)
[![MSRV: 1.85](https://img.shields.io/badge/MSRV-1.85-blue.svg)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0.html)

Rust bindings for [libome](https://gitlab.com/libome/libome/), a C++ library
implementing massive operator matrix elements (OMEs) of the QCD twist-2
operators in x-space.

This crate vendors the upstream C++ source and compiles it at build time — no
system-wide installation of libome is required.

## License and citation

The vendored C++ source in `native/libome/` is licensed under
**GPL-3.0-or-later** (see `native/libome/LICENSE`). Because this crate links
statically against that code, the combined work is also subject to the GPL-3.0
terms.

### Citation requirements

Users of this library must comply with the citation requirements of the
upstream libome project. The required references depend on which OMEs
(unpolarized and/or polarized) you use. The full list is in
[`CITATION`](CITATION), with BibTeX entries in
[`CITATION.bib`](CITATION.bib).

## Build requirements

- A C++17-capable compiler (GCC >= 7, Clang >= 5, MSVC >= 2017)
- Rust stable (edition 2024)
- The `cc` crate handles compilation; no CMake needed
- **Optional:** GSL (`libgsl-dev` / `gsl` via Homebrew) for the `mellin` feature

## Crate structure

```
native/libome/       Vendored upstream C++ source
build.rs             Compiles C++ into a static library via the cc crate
src/ffi.rs           Unsafe extern "C" declarations matching libome's C ABI
src/unpolarized/     Safe wrappers for unpolarized OMEs
src/polarized/       Safe wrappers for polarized (Delta) OMEs
src/mellin.rs        Mellin moment/convolution types (feature = "mellin")
src/gsl_shim.cpp     C++ shim bridging template-based Mellin API to C ABI
```

### Exposed OMEs

**Unpolarized** (9 matrix elements):

| Struct | C prefix | Parts |
|---|---|---|
| `AqqQNSEven` | `ome_AqqQNSEven_` | reg, plus, delta |
| `AqqQNSOdd` | `ome_AqqQNSOdd_` | reg, plus, delta |
| `AggQ` | `ome_AggQ_` | reg, plus, delta |
| `AQqPS` | `ome_AQqPS_` | reg |
| `AQqPSs` | `ome_AQqPSs_` | reg |
| `AqqQPS` | `ome_AqqQPS_` | reg |
| `AqgQ` | `ome_AqgQ_` | reg |
| `AgqQ` | `ome_AgqQ_` | reg |
| `AQg` | `ome_AQg_` | reg |

**Polarized** variants have the same structure, prefixed with `Pol` (e.g.
`PolAqqQNSEven`).

## Installation

### From crates.io

    [dependencies]
    libome-rs = "0.0.1"

With Mellin moment support:

    [dependencies]
    libome-rs = { version = "0.0.1", features = ["mellin"] }

The `mellin` feature requires system GSL, for example `libgsl-dev` on Debian/Ubuntu or `gsl` via Homebrew on macOS.

### From GitHub during development

    [dependencies]
    libome-rs = { git = "https://github.com/t7phy/libome_rs" }

With Mellin support:

    [dependencies]
    libome-rs = { git = "https://github.com/t7phy/libome_rs", features = ["mellin"] }

## CLI usage

```sh
# Evaluate the regular part of AqqQNSEven
libome-rs eval AqqQNSEven reg 0.25 -5.0 3.0 0.1

# Evaluate the plus part
libome-rs eval AqqQNSEven plus 0.25 -5.0 3.0 0.5

# Evaluate the delta part (no x argument)
libome-rs eval AqqQNSEven delta 0.25 -5.0 3.0

# Truncated evaluation in a_s at order 2
libome-rs eval AqqQNSEven reg-trunc 2 0.25 -5.0 3.0 0.1

# Mellin moment (requires --features mellin at install time)
libome-rs moment AQqPS 2 0.25 -5.0 3.0
```

## Library usage

```rust
use libome_rs::AqqQNSEven;

let as_ = 0.118 / (4.0 * std::f64::consts::PI); // a_s
let lm = 0.0;   // L_M = ln(m^2/mu^2)
let nf = 5.0;   // number of light flavours
let x = 0.1;    // momentum fraction

// Full evaluation
let reg = AqqQNSEven::reg(as_, lm, nf, x);

// Truncated in a_s at a given order
let trunc = AqqQNSEven::reg_trunc_as(2, as_, lm, nf, x);

// Individual coefficient
let coeff = AqqQNSEven::reg_coeff_as(2, lm, nf, x);

// Power ranges for iteration
let as_min = AqqQNSEven::reg_min_power();
let as_max = AqqQNSEven::reg_max_power();
```

The low-level numerical wrappers assume physically meaningful inputs, in particular `0 < x < 1` where an x-space regular or plus part is evaluated. Invalid domain inputs are passed to the upstream implementation and may produce NaN, infinities, or upstream-defined behavior.

## Feature: `mellin`

Enable the `mellin` feature to get Mellin moment and convolution helpers.
This links against system GSL (`libgsl-dev` on Debian/Ubuntu, `gsl` on macOS).

```toml
[dependencies]
libome-rs = { git = "https://github.com/t7phy/libome_rs", locked = true, features = ["mellin"] }
```

```rust
use libome_rs::{AqqQNSEven, IntegrationStatus};

let res = AqqQNSEven::mellin_moment(
    2,     // Mellin moment N
    0.25,  // a_s
    -5.0,  // L_M
    3.0,   // N_F
    0.0,   // eps_abs
    3e-11, // eps_rel
);
assert_eq!(res.status, IntegrationStatus::Success);
println!("N=2 moment: {} +/- {}", res.value, res.abs_error);

// Convolution with a user-provided test function
unsafe extern "C" fn f(x: f64) -> f64 { x }

let conv = AqqQNSEven::mellin_convolution(
    0.5,   // x
    0.25,  // a_s
    -5.0,  // L_M
    3.0,   // N_F
    f,     // test function
    0.0,   // eps_abs
    1e-6,  // eps_rel
);
```

## Running tests

```sh
cargo test                  # base tests (no GSL required)
cargo test --features mellin   # includes Mellin moment/convolution tests
```
