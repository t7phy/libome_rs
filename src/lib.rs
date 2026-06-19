//! # libome-rs
//!
//! Rust bindings for [libome](https://gitlab.com/libome/libome/), a C++ library
//! implementing massive operator matrix elements (OMEs) of the QCD twist-2
//! operators in x-space.
//!
//! ## What are OMEs?
//!
//! Operator matrix elements describe the transition between partonic states in
//! quantum chromodynamics (QCD). They appear in the variable flavour number
//! scheme (VFNS) for deep-inelastic scattering and are essential for matching
//! parton distribution functions (PDFs) across heavy-quark thresholds.
//!
//! Each OME is a function of:
//! - **`as_`** — the strong coupling constant `a_s(μ) = α_s(μ) / (4π)`
//! - **`lm`** — the mass logarithm `L_M = ln(m² / μ²)`
//! - **`nf`** — the number of massless quark flavours
//! - **`x`** — the momentum fraction (Bjorken-x)
//!
//! ## Distribution structure
//!
//! OMEs are distributions in `x` with up to three parts:
//!
//! - **Regular** (`reg`) — a smooth function of `x`, present for all OMEs.
//! - **Plus** (`plus`) — a plus-distribution `[f(x)]₊` proportional to
//!   `log^k(1-x) / (1-x)`, present only for
//!   [`AqqQNSEven`], [`AqqQNSOdd`], [`AggQ`] and their polarized variants.
//! - **Delta** (`delta`) — a `δ(1-x)` contribution (independent of `x`),
//!   present alongside `plus`.
//!
//! ## Available OMEs
//!
//! ### Unpolarized
//!
//! | Struct | Physics notation | Parts |
//! |---|---|---|
//! | [`AqqQNSEven`] | A_{qq,Q}^{NS,+} | reg, plus, delta |
//! | [`AqqQNSOdd`] | A_{qq,Q}^{NS,−} | reg, plus, delta |
//! | [`AggQ`] | A_{gg,Q} | reg, plus, delta |
//! | [`AQqPS`] | A_{Qq}^{PS} | reg |
//! | [`AQqPSs`] | A_{Qq}^{PS,s} | reg |
//! | [`AqqQPS`] | A_{qq,Q}^{PS} | reg |
//! | [`AqgQ`] | A_{qg,Q} | reg |
//! | [`AgqQ`] | A_{gq,Q} | reg |
//! | [`AQg`] | A_{Qg} | reg |
//!
//! ### Polarized
//!
//! The polarized variants carry a `Pol` prefix and correspond to the
//! ΔA matrix elements: [`PolAqqQNSEven`], [`PolAqqQNSOdd`], [`PolAggQ`],
//! [`PolAQqPS`], [`PolAQqPSs`], [`PolAqqQPS`], [`PolAqgQ`], [`PolAgqQ`],
//! [`PolAQg`].
//!
//! ## Quick start
//!
//! ```
//! use libome_rs::AqqQNSEven;
//!
//! let as_ = 0.118 / (4.0 * std::f64::consts::PI); // a_s
//! let lm = 0.0;   // L_M = ln(m²/μ²)
//! let nf = 5.0;   // number of light flavours
//! let x = 0.1;    // momentum fraction
//!
//! // Full evaluation of the regular part
//! let reg = AqqQNSEven::reg(as_, lm, nf, x);
//!
//! // Truncated in a_s at order 2
//! let trunc = AqqQNSEven::reg_trunc_as(2, as_, lm, nf, x);
//!
//! // Individual a_s coefficient
//! let coeff = AqqQNSEven::reg_coeff_as(2, lm, nf, x);
//!
//! // Plus and delta parts (only for RPD OMEs)
//! let plus = AqqQNSEven::plus(as_, lm, nf, x);
//! let delta = AqqQNSEven::delta(as_, lm, nf);
//! ```
//!
//! ## Iterating over coefficients
//!
//! Each part exposes power-range getters for systematic iteration:
//!
//! ```
//! use libome_rs::AggQ;
//!
//! for order_as in AggQ::reg_min_power()..=AggQ::reg_max_power() {
//!     for order_lm in AggQ::reg_coeff_as_min_power(order_as)
//!                   ..=AggQ::reg_coeff_as_max_power(order_as) {
//!         for order_nf in AggQ::reg_coeff_as_lm_min_power(order_as, order_lm)
//!                       ..=AggQ::reg_coeff_as_lm_max_power(order_as, order_lm) {
//!             let c = AggQ::reg_coeff_as_lm_nf(order_as, order_lm, order_nf, 0.5);
//!             // use coefficient c
//!         }
//!     }
//! }
//! ```
//!
//! ## Feature: `mellin`
//!
//! Enable the `mellin` feature to compute Mellin moments and convolutions via
//! numerical integration (requires system GSL at build time).
//!
//! ```toml
//! [dependencies]
//! libome-rs = { git = "https://github.com/t7phy/libome_rs", locked = true, features = ["mellin"] }
//! ```
//!
//! ```ignore
//! use libome_rs::{AQqPS, IntegrationStatus};
//!
//! let res = AQqPS::mellin_moment(2, 0.25, -5.0, 3.0, 0.0, 3e-11);
//! assert_eq!(res.status, IntegrationStatus::Success);
//! println!("N=2 moment: {} ± {}", res.value, res.abs_error);
//! ```
//!
//! ## Citation
//!
//! Users of this library must comply with the citation requirements of the
//! upstream libome project. See `CITATION` and `CITATION.bib` in the
//! repository root for the full list of required references.

pub mod ffi;

#[cfg(feature = "mellin")]
#[macro_use]
pub mod mellin;

#[macro_use]
mod macros;

pub mod polarized;
pub mod unpolarized;

pub use polarized::{
    PolAQg, PolAQqPS, PolAQqPSs, PolAggQ, PolAgqQ, PolAqgQ, PolAqqQNSEven, PolAqqQNSOdd, PolAqqQPS,
};
pub use unpolarized::{AQg, AQqPS, AQqPSs, AggQ, AgqQ, AqgQ, AqqQNSEven, AqqQNSOdd, AqqQPS};

#[cfg(feature = "mellin")]
pub use mellin::{IntegrationResult, IntegrationStatus};
