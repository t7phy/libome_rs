# CLAUDE.md — libome-rs project spec

## What this project is

Rust bindings for [libome](https://gitlab.com/libome/libome/), a C++ library
implementing massive operator matrix elements (OMEs) of the QCD twist-2
operators in x-space. The upstream C++ source is vendored under `native/libome/`
and compiled at build time via the `cc` crate.

## Upstream tracking

See `UPSTREAM.md` for the exact upstream commit, version, file inventory, and
instructions for updating the vendored source.

## Architecture

```
Cargo.toml              Crate metadata, features, dependencies
build.rs                Compiles vendored C++ via cc; conditionally links GSL
LICENSE                 GPL-3.0-or-later (from upstream)
CITATION / CITATION.bib Citation requirements (from upstream)
native/libome/          Vendored upstream C++ source (git subtree)

src/
  lib.rs                Crate root; re-exports all OME structs
  main.rs               CLI binary (eval, moment commands)
  ffi.rs                Unsafe extern "C" declarations for all C ABI functions
  macros.rs             Macros: define_reg_only_ome!, define_rpd_ome!
  mellin.rs             IntegrationResult/IntegrationStatus types + mellin macros
                        (only compiled with feature = "mellin")
  gsl_shim.cpp          C++ shim: stamps out extern "C" wrappers for
                        make_mellin_moment / make_mellin_convolution per OME
  unpolarized/
    mod.rs              Module declarations + re-exports for 9 unpolarized OMEs
    aqq_q_ns_even.rs    AqqQNSEven  (rpd: reg + plus + delta)
    aqq_q_ns_odd.rs     AqqQNSOdd   (rpd: reg + plus + delta)
    agg_q.rs            AggQ        (rpd: reg + plus + delta)
    a_qq_ps.rs          AQqPS       (reg-only)
    a_qq_ps_s.rs        AQqPSs      (reg-only)
    aqq_q_ps.rs         AqqQPS      (reg-only)
    aqg_q.rs            AqgQ        (reg-only)
    agq_q.rs            AgqQ        (reg-only)
    a_qg.rs             AQg         (reg-only)
  polarized/
    mod.rs              Module declarations + re-exports for 9 polarized OMEs
    (same file layout as unpolarized/, struct names prefixed with Pol)

tests/
  xspace.rs             X-space evaluation tests with upstream reference values
  mellin.rs             Mellin moment/convolution tests (cfg(feature = "mellin"))
```

## Two kinds of OME

Every OME header exposes a C ABI. There are two shapes:

### reg-only (12 OMEs)
AQqPS, AQqPSs, AqqQPS, AqgQ, AgqQ, AQg + their `pol` variants.

C functions per OME (11 total):
- `ome_<NAME>_reg(as, LM, NF, x) -> double`
- `ome_<NAME>_reg_trunc_as(order, as, LM, NF, x) -> double`
- `ome_<NAME>_reg_coeff_as(order_as, LM, NF, x) -> double`
- `ome_<NAME>_reg_coeff_as_LM(order_as, order_LM, NF, x) -> double`
- `ome_<NAME>_reg_coeff_as_LM_NF(order_as, order_LM, order_NF, x) -> double`
- `ome_<NAME>_reg_min_power() -> int`
- `ome_<NAME>_reg_max_power() -> int`
- `ome_<NAME>_reg_coeff_as_min_power(order_as) -> int`
- `ome_<NAME>_reg_coeff_as_max_power(order_as) -> int`
- `ome_<NAME>_reg_coeff_as_LM_min_power(order_as, order_LM) -> int`
- `ome_<NAME>_reg_coeff_as_LM_max_power(order_as, order_LM) -> int`

Rust: `define_reg_only_ome!(StructName, c_prefix)` stamps out all methods.

### rpd (6 OMEs)
AqqQNSEven, AqqQNSOdd, AggQ + their `pol` variants.

Same 11 reg functions, plus identical sets for `plus` (11 functions) and
`delta` (11 functions, but delta has no `x` parameter). 33 C functions total.

Rust: `define_rpd_ome!(StructName, c_prefix)` stamps out all methods.

## Feature: `mellin`

Adds Mellin moment and convolution support. Requires system GSL at build time.

- `build.rs` additionally compiles `integration_engine_gsl.cpp` and
  `src/gsl_shim.cpp`, links GSL via `pkg-config`.
- `gsl_shim.cpp` uses macros to stamp out `extern "C"` functions:
  - `ome_mellin_moment_<NAME>(n, as, lm, nf, eps_abs, eps_rel)` per OME
  - `ome_mellin_convolution_<NAME>(x, as, lm, nf, testfunc_ptr, eps_abs, eps_rel)` per OME
- `ffi.rs` declares these behind `#[cfg(feature = "mellin")]`
- `mellin.rs` defines `IntegrationStatus`, `IntegrationResult`, and
  `mellin_moment_method!` / `mellin_convolution_method!` macros
- `macros.rs` conditionally adds `mellin_moment()` and `mellin_convolution()`
  methods to every OME struct

## CLI binary (`src/main.rs`)

Commands:
- `libome-rs eval <OME> reg|plus|delta|reg-trunc|plus-trunc|delta-trunc ...`
- `libome-rs moment <OME> <N> <as> <LM> <NF> [eps_abs] [eps_rel]`
  (only with feature `mellin`)

Uses `dispatch_reg!` macro (all 18 OMEs) for reg operations and
`dispatch_rpd!` macro (6 RPD OMEs only) for plus/delta operations.

## How to add a new OME

When upstream adds a new OME (e.g. `NewOme` with header `NewOme.h` and
implementation `NewOme.cpp`):

1. **build.rs**: Add `"ome/NewOme.cpp"` to `cpp_files`.
2. **ffi.rs**: Add `extern "C"` declarations matching the C ABI in `NewOme.h`.
   Check whether it's reg-only or rpd (look for `_plus`/`_delta` functions).
3. **Wrapper module**: Create `src/unpolarized/new_ome.rs` (or `polarized/`)
   with `define_reg_only_ome!(NewOme, NewOme)` or
   `define_rpd_ome!(NewOme, NewOme)`.
4. **mod.rs**: Add `pub mod new_ome;` and `pub use new_ome::NewOme;` in the
   appropriate `mod.rs`.
5. **lib.rs**: Add `NewOme` to the `pub use` line.
6. **gsl_shim.cpp**: Add `MELLIN_FNS(NewOme)`.
7. **ffi.rs** (mellin section): Add `ome_mellin_moment_NewOme` and
   `ome_mellin_convolution_NewOme` declarations.
8. **main.rs**: Add the OME to `dispatch_reg!` (and `dispatch_rpd!` if rpd).
9. **Tests**: Add reference-value tests from upstream test file to
   `tests/xspace.rs` and `tests/mellin.rs`.
10. **UPSTREAM.md**: Update the file inventory.

## Dependencies

- `cc` (build): C++ compilation
- `pkg-config` (build): GSL discovery (mellin feature only)
- `paste` (runtime): Macro identifier concatenation

## Test commands

```sh
cargo test                      # base tests (38)
cargo test --features mellin    # all tests (56)
```
