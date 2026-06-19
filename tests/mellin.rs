#![cfg(feature = "mellin")]
#![allow(clippy::excessive_precision)]

// Mellin moment tests ported from upstream libome GoogleTest suite.
// Reference values from native/libome/tests/*.test.cpp (Nspace tests).
// Common parameters: as=0.25, LM=-5, NF=3.

use libome_rs::*;

const AS: f64 = 0.25;
const LM: f64 = -5.0;
const NF: f64 = 3.0;

fn check_moment(res: IntegrationResult, ref_val: f64, rel_tol: f64, label: &str) {
    assert_eq!(
        res.status,
        IntegrationStatus::Success,
        "{label}: integration failed with status {:?}",
        res.status
    );
    let err_bound = ref_val.abs() * rel_tol;
    let diff = (res.value - ref_val).abs();
    assert!(
        diff <= err_bound,
        "{label}: expected {ref_val}, got {}, diff {diff} > bound {err_bound}",
        res.value
    );
    assert!(
        res.abs_error <= err_bound,
        "{label}: abs_error {} > bound {err_bound}",
        res.abs_error
    );
}

// ═══════════════════ AQqPS Mellin moments ═══════════════════
// From native/libome/tests/AQqPS.test.cpp: AQqPSNspace,FullMoments

#[test]
fn a_qq_ps_full_moment_n2() {
    let res = AQqPS::mellin_moment(2, AS, LM, NF, 0.0, 3e-11);
    check_moment(res, -21.9987970416932404460059955113, 3e-11, "AQqPS N=2");
}

#[test]
fn a_qq_ps_full_moment_n4() {
    let res = AQqPS::mellin_moment(4, AS, LM, NF, 0.0, 3e-11);
    check_moment(res, -4.77399482861080891205508206879, 3e-11, "AQqPS N=4");
}

#[test]
fn a_qq_ps_full_moment_n6() {
    let res = AQqPS::mellin_moment(6, AS, LM, NF, 0.0, 3e-11);
    check_moment(res, -2.05321796632440898501494923712, 3e-11, "AQqPS N=6");
}

#[test]
fn a_qq_ps_full_moment_n8() {
    let res = AQqPS::mellin_moment(8, AS, LM, NF, 0.0, 3e-11);
    check_moment(res, -1.13668634294632596834371488214, 3e-11, "AQqPS N=8");
}

// ═══════════════════ AqqQNSEven Mellin moments ═══════════════════
// From native/libome/tests/AqqQNSEven.test.cpp: AqqQNSEvenNspace,FullMoments

#[test]
fn aqq_q_ns_even_full_moment_n2() {
    let res = AqqQNSEven::mellin_moment(2, AS, LM, NF, 0.0, 3e-11);
    check_moment(
        res,
        -15.4108510022264770011147721835,
        3e-11,
        "AqqQNSEven N=2",
    );
}

#[test]
fn aqq_q_ns_even_full_moment_n4() {
    let res = AqqQNSEven::mellin_moment(4, AS, LM, NF, 0.0, 3e-11);
    check_moment(
        res,
        -28.7584662796242410546737975434,
        3e-11,
        "AqqQNSEven N=4",
    );
}

#[test]
fn aqq_q_ns_even_full_moment_n6() {
    let res = AqqQNSEven::mellin_moment(6, AS, LM, NF, 0.0, 3e-11);
    check_moment(
        res,
        -36.4195966475157942743467607082,
        3e-11,
        "AqqQNSEven N=6",
    );
}

#[test]
fn aqq_q_ns_even_full_moment_n8() {
    let res = AqqQNSEven::mellin_moment(8, AS, LM, NF, 0.0, 3e-11);
    check_moment(
        res,
        -41.8758791904766359860067192509,
        3e-11,
        "AqqQNSEven N=8",
    );
}

// ═══════════════════ AggQ Mellin moments ═══════════════════
// From native/libome/tests/AggQ.test.cpp: AggQNspace,FullMoments

#[test]
fn agg_q_full_moment_n2() {
    let res = AggQ::mellin_moment(2, AS, LM, NF, 0.0, 3e-11);
    check_moment(res, -67.7252905373040287136420003452, 3e-11, "AggQ N=2");
}

#[test]
fn agg_q_full_moment_n4() {
    let res = AggQ::mellin_moment(4, AS, LM, NF, 0.0, 3e-11);
    check_moment(res, -104.842284451535213240184896169, 3e-11, "AggQ N=4");
}

// ═══════════════════ Reg-only OME moments ═══════════════════

#[test]
fn aqg_q_full_moment_n2() {
    let res = AqgQ::mellin_moment(2, AS, LM, NF, 0.0, 3e-11);
    assert_eq!(res.status, IntegrationStatus::Success);
    assert!(res.value.is_finite());
}

#[test]
fn agq_q_full_moment_n2() {
    let res = AgqQ::mellin_moment(2, AS, LM, NF, 0.0, 3e-11);
    assert_eq!(res.status, IntegrationStatus::Success);
    assert!(res.value.is_finite());
}

#[test]
fn a_qg_full_moment_n2() {
    let res = AQg::mellin_moment(2, AS, LM, NF, 0.0, 3e-11);
    assert_eq!(res.status, IntegrationStatus::Success);
    assert!(res.value.is_finite());
}

// ═══════════════════ Polarized moments ═══════════════════

#[test]
fn pol_aqq_q_ns_even_moment_n2() {
    let res = PolAqqQNSEven::mellin_moment(2, AS, LM, NF, 0.0, 3e-11);
    assert_eq!(res.status, IntegrationStatus::Success);
    assert!(res.value.is_finite());
}

#[test]
fn pol_agg_q_moment_n2() {
    let res = PolAggQ::mellin_moment(2, AS, LM, NF, 0.0, 3e-11);
    assert_eq!(res.status, IntegrationStatus::Success);
    assert!(res.value.is_finite());
}

#[test]
fn pol_a_qq_ps_moment_n2() {
    let res = PolAQqPS::mellin_moment(2, AS, LM, NF, 0.0, 3e-11);
    assert_eq!(res.status, IntegrationStatus::Success);
    assert!(res.value.is_finite());
}

// ═══════════════════ Convolutions ═══════════════════

unsafe extern "C" fn identity(x: f64) -> f64 {
    x
}

#[test]
fn aqq_q_ns_even_convolution() {
    let res = AqqQNSEven::mellin_convolution(0.5, AS, LM, NF, identity, 0.0, 1e-6);
    assert_eq!(
        res.status,
        IntegrationStatus::Success,
        "convolution failed: {:?}",
        res.status
    );
    assert!(
        res.value.is_finite(),
        "convolution result not finite: {}",
        res.value
    );
}

#[test]
fn a_qq_ps_convolution() {
    let res = AQqPS::mellin_convolution(0.5, AS, LM, NF, identity, 0.0, 1e-6);
    assert_eq!(res.status, IntegrationStatus::Success);
    assert!(res.value.is_finite());
}
