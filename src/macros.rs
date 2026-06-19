macro_rules! reg_methods {
    ($prefix:ident) => {
        paste::paste! {
            /// Evaluate the regular part as a function of all parameters.
            ///
            /// Computes the full sum over all powers of `a_s`, `L_M`, and `N_F`.
            ///
            /// # Parameters
            /// - `as_` — strong coupling `a_s(μ) = α_s(μ) / (4π)`
            /// - `lm` — mass logarithm `L_M = ln(m² / μ²)`
            /// - `nf` — number of massless quark flavours
            /// - `x` — momentum fraction (Bjorken-x), must be in (0, 1)
            pub fn reg(as_: f64, lm: f64, nf: f64, x: f64) -> f64 {
                unsafe { crate::ffi::[<ome_ $prefix _reg>](as_, lm, nf, x) }
            }

            /// Evaluate the regular part, truncated in `a_s` at the given order.
            ///
            /// Only includes powers of `a_s` up to and including `trunc_order`.
            pub fn reg_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64, x: f64) -> f64 {
                unsafe { crate::ffi::[<ome_ $prefix _reg_trunc_as>](trunc_order, as_, lm, nf, x) }
            }

            /// Evaluate a single `a_s` coefficient of the regular part.
            ///
            /// Returns the coefficient of `a_s^order_as`, which is still a
            /// function of `L_M`, `N_F`, and `x`.
            pub fn reg_coeff_as(order_as: i32, lm: f64, nf: f64, x: f64) -> f64 {
                unsafe { crate::ffi::[<ome_ $prefix _reg_coeff_as>](order_as, lm, nf, x) }
            }

            /// Evaluate a single `a_s`, `L_M` coefficient of the regular part.
            ///
            /// Returns the coefficient of `a_s^order_as * L_M^order_lm`, which
            /// is still a function of `N_F` and `x`.
            pub fn reg_coeff_as_lm(order_as: i32, order_lm: i32, nf: f64, x: f64) -> f64 {
                unsafe { crate::ffi::[<ome_ $prefix _reg_coeff_as_LM>](order_as, order_lm, nf, x) }
            }

            /// Evaluate a single `a_s`, `L_M`, `N_F` coefficient of the regular part.
            ///
            /// Returns the coefficient of `a_s^order_as * L_M^order_lm * N_F^order_nf`,
            /// which is a function of `x` only.
            pub fn reg_coeff_as_lm_nf(order_as: i32, order_lm: i32, order_nf: i32, x: f64) -> f64 {
                unsafe { crate::ffi::[<ome_ $prefix _reg_coeff_as_LM_NF>](order_as, order_lm, order_nf, x) }
            }

            /// Minimum power of `a_s` in the regular part.
            pub fn reg_min_power() -> i32 {
                unsafe { crate::ffi::[<ome_ $prefix _reg_min_power>]() }
            }

            /// Maximum power of `a_s` in the regular part.
            pub fn reg_max_power() -> i32 {
                unsafe { crate::ffi::[<ome_ $prefix _reg_max_power>]() }
            }

            /// Minimum power of `L_M` for a given `a_s` coefficient of the regular part.
            pub fn reg_coeff_as_min_power(order_as: i32) -> i32 {
                unsafe { crate::ffi::[<ome_ $prefix _reg_coeff_as_min_power>](order_as) }
            }

            /// Maximum power of `L_M` for a given `a_s` coefficient of the regular part.
            pub fn reg_coeff_as_max_power(order_as: i32) -> i32 {
                unsafe { crate::ffi::[<ome_ $prefix _reg_coeff_as_max_power>](order_as) }
            }

            /// Minimum power of `N_F` for a given `a_s`, `L_M` coefficient of the regular part.
            pub fn reg_coeff_as_lm_min_power(order_as: i32, order_lm: i32) -> i32 {
                unsafe { crate::ffi::[<ome_ $prefix _reg_coeff_as_LM_min_power>](order_as, order_lm) }
            }

            /// Maximum power of `N_F` for a given `a_s`, `L_M` coefficient of the regular part.
            pub fn reg_coeff_as_lm_max_power(order_as: i32, order_lm: i32) -> i32 {
                unsafe { crate::ffi::[<ome_ $prefix _reg_coeff_as_LM_max_power>](order_as, order_lm) }
            }
        }
    };
}

macro_rules! plus_methods {
    ($prefix:ident) => {
        paste::paste! {
            /// Evaluate the plus-distribution part as a function of all parameters.
            ///
            /// The plus part models `[f₊(x)]₊`, a sum of plus-distribution
            /// kernels `log^k(1-x) / (1-x)`. This is the kernel value before
            /// integration against a test function.
            ///
            /// # Parameters
            /// - `as_` — strong coupling `a_s(μ) = α_s(μ) / (4π)`
            /// - `lm` — mass logarithm `L_M = ln(m² / μ²)`
            /// - `nf` — number of massless quark flavours
            /// - `x` — momentum fraction, must be in (0, 1)
            pub fn plus(as_: f64, lm: f64, nf: f64, x: f64) -> f64 {
                unsafe { crate::ffi::[<ome_ $prefix _plus>](as_, lm, nf, x) }
            }

            /// Evaluate the plus part, truncated in `a_s` at the given order.
            pub fn plus_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64, x: f64) -> f64 {
                unsafe { crate::ffi::[<ome_ $prefix _plus_trunc_as>](trunc_order, as_, lm, nf, x) }
            }

            /// Evaluate a single `a_s` coefficient of the plus part.
            pub fn plus_coeff_as(order_as: i32, lm: f64, nf: f64, x: f64) -> f64 {
                unsafe { crate::ffi::[<ome_ $prefix _plus_coeff_as>](order_as, lm, nf, x) }
            }

            /// Evaluate a single `a_s`, `L_M` coefficient of the plus part.
            pub fn plus_coeff_as_lm(order_as: i32, order_lm: i32, nf: f64, x: f64) -> f64 {
                unsafe { crate::ffi::[<ome_ $prefix _plus_coeff_as_LM>](order_as, order_lm, nf, x) }
            }

            /// Evaluate a single `a_s`, `L_M`, `N_F` coefficient of the plus part.
            pub fn plus_coeff_as_lm_nf(order_as: i32, order_lm: i32, order_nf: i32, x: f64) -> f64 {
                unsafe { crate::ffi::[<ome_ $prefix _plus_coeff_as_LM_NF>](order_as, order_lm, order_nf, x) }
            }

            /// Minimum power of `a_s` in the plus part.
            pub fn plus_min_power() -> i32 {
                unsafe { crate::ffi::[<ome_ $prefix _plus_min_power>]() }
            }

            /// Maximum power of `a_s` in the plus part.
            pub fn plus_max_power() -> i32 {
                unsafe { crate::ffi::[<ome_ $prefix _plus_max_power>]() }
            }

            /// Minimum power of `L_M` for a given `a_s` coefficient of the plus part.
            pub fn plus_coeff_as_min_power(order_as: i32) -> i32 {
                unsafe { crate::ffi::[<ome_ $prefix _plus_coeff_as_min_power>](order_as) }
            }

            /// Maximum power of `L_M` for a given `a_s` coefficient of the plus part.
            pub fn plus_coeff_as_max_power(order_as: i32) -> i32 {
                unsafe { crate::ffi::[<ome_ $prefix _plus_coeff_as_max_power>](order_as) }
            }

            /// Minimum power of `N_F` for a given `a_s`, `L_M` coefficient of the plus part.
            pub fn plus_coeff_as_lm_min_power(order_as: i32, order_lm: i32) -> i32 {
                unsafe { crate::ffi::[<ome_ $prefix _plus_coeff_as_LM_min_power>](order_as, order_lm) }
            }

            /// Maximum power of `N_F` for a given `a_s`, `L_M` coefficient of the plus part.
            pub fn plus_coeff_as_lm_max_power(order_as: i32, order_lm: i32) -> i32 {
                unsafe { crate::ffi::[<ome_ $prefix _plus_coeff_as_LM_max_power>](order_as, order_lm) }
            }
        }
    };
}

macro_rules! delta_methods {
    ($prefix:ident) => {
        paste::paste! {
            /// Evaluate the delta-distribution coefficient.
            ///
            /// The delta part is the coefficient of `δ(1-x)` and is independent
            /// of `x`. It contributes only at `x = 1`.
            ///
            /// # Parameters
            /// - `as_` — strong coupling `a_s(μ) = α_s(μ) / (4π)`
            /// - `lm` — mass logarithm `L_M = ln(m² / μ²)`
            /// - `nf` — number of massless quark flavours
            pub fn delta(as_: f64, lm: f64, nf: f64) -> f64 {
                unsafe { crate::ffi::[<ome_ $prefix _delta>](as_, lm, nf) }
            }

            /// Evaluate the delta part, truncated in `a_s` at the given order.
            pub fn delta_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64) -> f64 {
                unsafe { crate::ffi::[<ome_ $prefix _delta_trunc_as>](trunc_order, as_, lm, nf) }
            }

            /// Evaluate a single `a_s` coefficient of the delta part.
            pub fn delta_coeff_as(order_as: i32, lm: f64, nf: f64) -> f64 {
                unsafe { crate::ffi::[<ome_ $prefix _delta_coeff_as>](order_as, lm, nf) }
            }

            /// Evaluate a single `a_s`, `L_M` coefficient of the delta part.
            pub fn delta_coeff_as_lm(order_as: i32, order_lm: i32, nf: f64) -> f64 {
                unsafe { crate::ffi::[<ome_ $prefix _delta_coeff_as_LM>](order_as, order_lm, nf) }
            }

            /// Evaluate a single `a_s`, `L_M`, `N_F` coefficient of the delta part.
            pub fn delta_coeff_as_lm_nf(order_as: i32, order_lm: i32, order_nf: i32) -> f64 {
                unsafe { crate::ffi::[<ome_ $prefix _delta_coeff_as_LM_NF>](order_as, order_lm, order_nf) }
            }

            /// Minimum power of `a_s` in the delta part.
            pub fn delta_min_power() -> i32 {
                unsafe { crate::ffi::[<ome_ $prefix _delta_min_power>]() }
            }

            /// Maximum power of `a_s` in the delta part.
            pub fn delta_max_power() -> i32 {
                unsafe { crate::ffi::[<ome_ $prefix _delta_max_power>]() }
            }

            /// Minimum power of `L_M` for a given `a_s` coefficient of the delta part.
            pub fn delta_coeff_as_min_power(order_as: i32) -> i32 {
                unsafe { crate::ffi::[<ome_ $prefix _delta_coeff_as_min_power>](order_as) }
            }

            /// Maximum power of `L_M` for a given `a_s` coefficient of the delta part.
            pub fn delta_coeff_as_max_power(order_as: i32) -> i32 {
                unsafe { crate::ffi::[<ome_ $prefix _delta_coeff_as_max_power>](order_as) }
            }

            /// Minimum power of `N_F` for a given `a_s`, `L_M` coefficient of the delta part.
            pub fn delta_coeff_as_lm_min_power(order_as: i32, order_lm: i32) -> i32 {
                unsafe { crate::ffi::[<ome_ $prefix _delta_coeff_as_LM_min_power>](order_as, order_lm) }
            }

            /// Maximum power of `N_F` for a given `a_s`, `L_M` coefficient of the delta part.
            pub fn delta_coeff_as_lm_max_power(order_as: i32, order_lm: i32) -> i32 {
                unsafe { crate::ffi::[<ome_ $prefix _delta_coeff_as_LM_max_power>](order_as, order_lm) }
            }
        }
    };
}

macro_rules! define_reg_only_ome {
    ($(#[$meta:meta])* $name:ident, $prefix:ident) => {
        $(#[$meta])*
        pub struct $name;
        impl $name {
            reg_methods!($prefix);
        }
        #[cfg(feature = "mellin")]
        impl $name {
            mellin_moment_method!($prefix);
            mellin_convolution_method!($prefix);
        }
    };
}

macro_rules! define_rpd_ome {
    ($(#[$meta:meta])* $name:ident, $prefix:ident) => {
        $(#[$meta])*
        pub struct $name;
        impl $name {
            reg_methods!($prefix);
            plus_methods!($prefix);
            delta_methods!($prefix);
        }
        #[cfg(feature = "mellin")]
        impl $name {
            mellin_moment_method!($prefix);
            mellin_convolution_method!($prefix);
        }
    };
}
