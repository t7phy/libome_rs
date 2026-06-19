//! Mellin moment and convolution support (requires feature `mellin`).
//!
//! This module provides numerical integration routines for computing Mellin
//! moments and Mellin convolutions of the OME distributions, backed by the
//! GNU Scientific Library (GSL) CQUAD adaptive integrator.
//!
//! When the `mellin` feature is enabled, every OME struct gains two additional
//! methods: [`mellin_moment`] and [`mellin_convolution`].
//!
//! [`mellin_moment`]: crate::AqqQNSEven::mellin_moment
//! [`mellin_convolution`]: crate::AqqQNSEven::mellin_convolution

/// Status code returned by the GSL numerical integrator.
///
/// Maps directly to the upstream `ome::integration_status` enum.
/// [`Success`](IntegrationStatus::Success) indicates that the requested
/// precision was reached; all other variants indicate a problem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum IntegrationStatus {
    /// Integration completed successfully within the requested tolerance.
    Success = 0,
    /// The adaptive integrator reached its maximum number of subdivisions.
    MaxIterationsReached = 1,
    /// Rounding errors prevent the requested precision from being achieved.
    RoundingErrorDetected = 2,
    /// A non-integrable singularity was detected in the integration interval.
    SingularityDetected = 3,
    /// The integral is divergent or converges too slowly.
    DivergenceDetected = 4,
    /// The requested precision could not be reached.
    PrecisionNotReached = 5,
    /// An argument to the integrator had an invalid value.
    DomainError = 6,
    /// The specified tolerance is invalid.
    BadTolerance = 7,
    /// An unclassified error from the GSL integrator.
    OtherError = 8,
}

impl IntegrationStatus {
    fn from_raw(v: i32) -> Self {
        match v {
            0 => Self::Success,
            1 => Self::MaxIterationsReached,
            2 => Self::RoundingErrorDetected,
            3 => Self::SingularityDetected,
            4 => Self::DivergenceDetected,
            5 => Self::PrecisionNotReached,
            6 => Self::DomainError,
            7 => Self::BadTolerance,
            _ => Self::OtherError,
        }
    }
}

/// Result of a numerical integration (Mellin moment or convolution).
///
/// Contains the integration [`status`](IntegrationResult::status), the
/// computed [`value`](IntegrationResult::value), and an estimate of the
/// [`abs_error`](IntegrationResult::abs_error).
///
/// # Example
///
/// ```ignore
/// use libome_rs::{AQqPS, IntegrationStatus};
///
/// let res = AQqPS::mellin_moment(2, 0.25, -5.0, 3.0, 0.0, 3e-11);
/// assert_eq!(res.status, IntegrationStatus::Success);
/// println!("value = {} ± {}", res.value, res.abs_error);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct IntegrationResult {
    /// Whether the integration succeeded or encountered a problem.
    pub status: IntegrationStatus,
    /// The computed value of the integral.
    pub value: f64,
    /// Estimate of the absolute error on [`value`](IntegrationResult::value).
    pub abs_error: f64,
}

impl IntegrationResult {
    pub(crate) fn from_raw(r: crate::ffi::OmeIntegrationResult) -> Self {
        Self {
            status: IntegrationStatus::from_raw(r.status),
            value: r.result,
            abs_error: r.abs_error,
        }
    }
}

macro_rules! mellin_moment_method {
    ($prefix:ident) => {
        paste::paste! {
            /// Compute the `n`-th Mellin moment of this OME.
            ///
            /// The Mellin moment is defined as:
            ///
            /// ```text
            /// M(n) = ∫₀¹ dx x^(n-1) [f_reg(x) + [f₊(x)]₊ + δ(1-x) f_δ]
            /// ```
            ///
            /// where the plus and delta contributions are included automatically
            /// for OMEs that have them.
            ///
            /// # Parameters
            /// - `n` — Mellin moment index (positive integer)
            /// - `as_` — strong coupling `a_s(μ)`
            /// - `lm` — mass logarithm `L_M`
            /// - `nf` — number of massless quark flavours
            /// - `eps_abs` — requested absolute integration error (use `0.0`
            ///   to rely on `eps_rel` alone)
            /// - `eps_rel` — requested relative integration error (e.g. `3e-11`)
            ///
            /// # Returns
            ///
            /// An [`IntegrationResult`](crate::IntegrationResult) with the
            /// status, value, and absolute error estimate.
            pub fn mellin_moment(
                n: i32,
                as_: f64,
                lm: f64,
                nf: f64,
                eps_abs: f64,
                eps_rel: f64,
            ) -> crate::mellin::IntegrationResult {
                crate::mellin::IntegrationResult::from_raw(unsafe {
                    crate::ffi::[<ome_mellin_moment_ $prefix>](n, as_, lm, nf, eps_abs, eps_rel)
                })
            }
        }
    };
}

macro_rules! mellin_convolution_method {
    ($prefix:ident) => {
        paste::paste! {
            /// Compute the Mellin convolution of this OME with a test function.
            ///
            /// The Mellin convolution is defined as:
            ///
            /// ```text
            /// (f ⊗ g)(x) = ∫_x^1 dz/z f_reg(z) g(x/z)
            ///             + ∫_x^1 dz f₊(z) [g(x/z)/z - g(x)]
            ///             - ∫_0^x dz f₊(z) g(x)
            ///             + f_δ g(x)
            /// ```
            ///
            /// # Parameters
            /// - `x` — momentum fraction at which to evaluate the convolution
            /// - `as_` — strong coupling `a_s(μ)`
            /// - `lm` — mass logarithm `L_M`
            /// - `nf` — number of massless quark flavours
            /// - `testfunc` — the test function `g(x)`, provided as an
            ///   `unsafe extern "C" fn(f64) -> f64`
            /// - `eps_abs` — requested absolute integration error
            /// - `eps_rel` — requested relative integration error
            ///
            /// # Returns
            ///
            /// An [`IntegrationResult`](crate::IntegrationResult) with the
            /// status, value, and absolute error estimate.
            pub fn mellin_convolution(
                x: f64,
                as_: f64,
                lm: f64,
                nf: f64,
                testfunc: unsafe extern "C" fn(f64) -> f64,
                eps_abs: f64,
                eps_rel: f64,
            ) -> crate::mellin::IntegrationResult {
                crate::mellin::IntegrationResult::from_raw(unsafe {
                    crate::ffi::[<ome_mellin_convolution_ $prefix>](x, as_, lm, nf, testfunc, eps_abs, eps_rel)
                })
            }
        }
    };
}
