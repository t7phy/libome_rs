# Upstream tracking — libome

This file records the state of the vendored upstream libome source so that
future updates can be diffed and applied.

## Source

- Repository: https://gitlab.com/libome/libome/
- Vendored at: `native/libome/`
- Method: git subtree (squashed)

## Last vendored

- Upstream commit: `f0dc6bf`
- Upstream version: `1.1.0` (from CMakeLists.txt)
- Date vendored: 2026-06-18
- Our merge commit: `a64f9e7`

## Upstream file inventory

### Headers (public API)

```
ome/ome.h                   Main include (includes everything below)
ome/ome_type_aliases.h      Template type aliases for OME nesting structure
ome/functions.h             Function wrappers (func_shift, func_apply, etc.)
ome/laurent_polynomial.h    Laurent polynomial template
ome/piecewise.h             Piecewise function template
ome/rpd_distribution.h      Regular + plus + delta distribution container
ome/traits.h                Type traits for template metaprogramming
ome/mellin.h                Mellin moment and convolution templates
ome/integration_engine.h    Abstract integration engine interface
ome/integration_engine_gsl.h  GSL-based integration engine
```

### OME headers + implementations (18 OMEs)

Unpolarized (9):
```
ome/AqqQNSEven.{h,cpp}     A_{qq,Q}^{NS,+}    [reg, plus, delta]
ome/AqqQNSOdd.{h,cpp}      A_{qq,Q}^{NS,-}     [reg, plus, delta]
ome/AggQ.{h,cpp}           A_{gg,Q}            [reg, plus, delta]
ome/AQqPS.{h,cpp}          A_{Qq}^{PS}         [reg only]
ome/AQqPSs.{h,cpp}         A_{Qq}^{PS,s}       [reg only]
ome/AqqQPS.{h,cpp}         A_{qq,Q}^{PS}       [reg only]
ome/AqgQ.{h,cpp}           A_{qg,Q}            [reg only]
ome/AgqQ.{h,cpp}           A_{gq,Q}            [reg only]
ome/AQg.{h,cpp}            A_{Qg}              [reg only]
```

Polarized (9):
```
ome/polAqqQNSEven.{h,cpp}  Delta A_{qq,Q}^{NS,+}  [reg, plus, delta]
ome/polAqqQNSOdd.{h,cpp}   Delta A_{qq,Q}^{NS,-}   [reg, plus, delta]
ome/polAggQ.{h,cpp}        Delta A_{gg,Q}          [reg, plus, delta]
ome/polAQqPS.{h,cpp}       Delta A_{Qq}^{PS}       [reg only]
ome/polAQqPSs.{h,cpp}      Delta A_{Qq}^{PS,s}     [reg only]
ome/polAqqQPS.{h,cpp}      Delta A_{qq,Q}^{PS}     [reg only]
ome/polAqgQ.{h,cpp}        Delta A_{qg,Q}          [reg only]
ome/polAgqQ.{h,cpp}        Delta A_{gq,Q}          [reg only]
ome/polAQg.{h,cpp}         Delta A_{Qg}            [reg only]
```

### Other upstream files

```
ome/integration_engine_gsl.cpp   GSL integration implementation
src/CMakeLists.txt               Upstream build (we use build.rs instead)
```

### Upstream test files

```
tests/AqqQNSEven.test.cpp    tests/polAqqQNSEven.test.cpp
tests/AqqQNSOdd.test.cpp     tests/polAqqQNSOdd.test.cpp
tests/AggQ.test.cpp          tests/polAggQ.test.cpp
tests/AQqPS.test.cpp         tests/polAQqPS.test.cpp
tests/AQqPSs.test.cpp        tests/polAQqPSs.test.cpp
tests/AqqQPS.test.cpp        tests/polAqqQPS.test.cpp
tests/AqgQ.test.cpp          tests/polAqgQ.test.cpp
tests/AgqQ.test.cpp          tests/polAgqQ.test.cpp
tests/AQg.test.cpp           tests/polAQg.test.cpp
tests/functions.test.cpp
tests/laurent_polynomial.test.cpp
tests/piecewise.test.cpp
tests/rpd_distribution.test.cpp
tests/mellin.test.cpp
tests/integration_engine_gsl.test.cpp
```

## How to update the vendored source

The initial import was done with:

```sh
git subtree add --prefix native/libome https://gitlab.com/libome/libome.git main --squash
```

To pull in upstream updates:

```sh
git subtree pull --prefix native/libome https://gitlab.com/libome/libome.git main --squash
```

After the subtree pull lands, ask Claude Code to sync the Rust bindings:

> "I have updated the vendored libome under native/libome/. Diff the
> current source against the inventory in UPSTREAM.md, add bindings for
> any new OMEs or changed C ABI signatures, update tests with upstream
> reference values, and update UPSTREAM.md."

Claude Code must never modify anything under `native/libome/`.

## What to look for in an upstream update

1. **New OME files**: New `*.{h,cpp}` pairs in `src/ome/`. Each needs full
   binding treatment (see "How to add a new OME" in CLAUDE.md).
2. **Changed C ABI signatures**: Grep for `extern "C"` in changed headers.
   Compare against declarations in `src/ffi.rs`.
3. **New or changed reference values in test files**: Port to
   `tests/xspace.rs` and `tests/mellin.rs`.
4. **New header-only templates**: May require new shim functions in
   `src/gsl_shim.cpp` if they add Mellin-related functionality.
5. **Version bump**: Update the version in this file.
6. **New CITATION entries**: Update `CITATION` and `CITATION.bib` at root.
7. **New features, dependencies, or build requirements**: Check for changes
   in `CMakeLists.txt` (new `find_package`, new options, new subdirectories),
   new header-only modules, new example programs, or new external
   dependencies. Report any such changes to the user and ask whether
   libome-rs should be extended to cover them.
