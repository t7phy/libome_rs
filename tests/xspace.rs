#![allow(clippy::excessive_precision)]

// Tests ported from upstream libome GoogleTest suite.
// Reference values come from native/libome/tests/*.test.cpp.
// Common parameters: as=0.25, LM=-5, NF=3.

use libome_rs::*;

const AS: f64 = 0.25;
const LM: f64 = -5.0;
const NF: f64 = 3.0;
const EPS: f64 = f64::EPSILON;

fn err_bound(ref_val: f64) -> f64 {
    ref_val.abs() * 2048.0 * EPS
}

fn err_bound_near_one(ref_val: f64, x: f64) -> f64 {
    ref_val.abs() * f64::max(EPS / (1.0 - x), 2048.0 * EPS)
}

fn assert_near(actual: f64, expected: f64, tol: f64, label: &str) {
    let diff = (actual - expected).abs();
    assert!(
        diff <= tol,
        "{label}: expected {expected}, got {actual}, diff {diff} > tol {tol}"
    );
}

// ═══════════════════ AqqQNSEven (reg + plus + delta) ═══════════════════

#[test]
fn aqq_q_ns_even_full_delta() {
    let v = AqqQNSEven::delta(AS, LM, NF);
    assert!(
        (v - 12.1501748418929015300593526677).abs() <= 4.0 * EPS * 12.1501748418929015300593526677,
        "AqqQNSEven delta: got {v}"
    );
}

#[test]
fn aqq_q_ns_even_full_reg_powers_of_4() {
    let cases: &[(f64, f64)] = &[
        (9.31322574615478515625e-10, 2163.01257897213413594095782306),
        (0.0625, 0.137089640189828775255456845247),
        (0.9375, -20.5606144911050942416354711787),
        (
            0.999999999068677425384521484375,
            -204.01251843569601974891835882,
        ),
    ];
    for &(x, ref_val) in cases {
        let v = AqqQNSEven::reg(AS, LM, NF, x);
        assert_near(
            v,
            ref_val,
            err_bound(ref_val),
            &format!("AqqQNSEven reg x={x}"),
        );
    }
}

#[test]
fn aqq_q_ns_even_full_reg_normal_doubles() {
    let cases: &[(f64, f64)] = &[
        (0.1, -3.18596704259147533517278785729),
        (0.5, -12.6331637229158185956132973196),
        (0.9, -18.8972597634602817291367015833),
        (0.9999999999, -258.186528604675458297909307061),
    ];
    for &(x, ref_val) in cases {
        let v = AqqQNSEven::reg(AS, LM, NF, x);
        assert_near(
            v,
            ref_val,
            err_bound_near_one(ref_val, x),
            &format!("AqqQNSEven reg x={x}"),
        );
    }
}

#[test]
fn aqq_q_ns_even_full_plus_normal_doubles() {
    let cases: &[(f64, f64)] = &[
        (0.1, 22.0531821516420224050988718576),
        (0.5, 39.6957278729556403291779693437),
        (0.9, 198.478639364778201645889846718),
        (0.99, 1984.78639364778201645889846718),
    ];
    for &(x, ref_val) in cases {
        let v = AqqQNSEven::plus(AS, LM, NF, x);
        assert_near(
            v,
            ref_val,
            err_bound_near_one(ref_val, x),
            &format!("AqqQNSEven plus x={x}"),
        );
    }
}

#[test]
fn aqq_q_ns_even_trunc_as2_reg() {
    let cases: &[(f64, f64)] = &[
        (9.31322574615478515625e-10, 17.8217061035486357547375500369),
        (0.0625, -0.177135651603830542552739162964),
        (0.9375, -0.38499958453489546857703568513),
    ];
    for &(x, ref_val) in cases {
        let v = AqqQNSEven::reg_trunc_as(2, AS, LM, NF, x);
        assert_near(
            v,
            ref_val,
            err_bound(ref_val),
            &format!("AqqQNSEven trunc_as2 reg x={x}"),
        );
    }
}

#[test]
fn aqq_q_ns_even_trunc_as0_delta() {
    let v = AqqQNSEven::delta_trunc_as(0, AS, LM, NF);
    assert!(
        (v - 1.0).abs() <= 4.0 * EPS,
        "AqqQNSEven trunc_as0 delta: got {v}"
    );
}

// ═══════════════════ AqqQNSOdd (reg + plus + delta) ═══════════════════

#[test]
fn aqq_q_ns_odd_full_delta() {
    let v = AqqQNSOdd::delta(AS, LM, NF);
    assert!(
        (v - 12.1501748418929015300593526677).abs() <= 4.0 * EPS * 12.1501748418929015300593526677,
        "AqqQNSOdd delta: got {v}"
    );
}

#[test]
fn aqq_q_ns_odd_full_reg_normal_doubles() {
    let cases: &[(f64, f64)] = &[
        (0.1, -2.64779113163787358652265882781),
        (0.5, -12.6750864156939643451842303613),
        (0.9, -18.9079442394228220141994118981),
    ];
    for &(x, ref_val) in cases {
        let v = AqqQNSOdd::reg(AS, LM, NF, x);
        assert_near(
            v,
            ref_val,
            err_bound_near_one(ref_val, x),
            &format!("AqqQNSOdd reg x={x}"),
        );
    }
}

#[test]
fn aqq_q_ns_odd_full_plus_powers_of_4() {
    let cases: &[(f64, f64)] = &[
        (0.0625, 21.1710548655763415088949169833),
        (0.9375, 317.565822983645122633423754749),
    ];
    for &(x, ref_val) in cases {
        let v = AqqQNSOdd::plus(AS, LM, NF, x);
        assert_near(
            v,
            ref_val,
            err_bound(ref_val),
            &format!("AqqQNSOdd plus x={x}"),
        );
    }
}

// ═══════════════════ AggQ (reg + plus + delta) ═══════════════════

#[test]
fn agg_q_full_delta() {
    let v = AggQ::delta(AS, LM, NF);
    assert!(
        (v - (-13.005021117466894651792463195)).abs() <= 4.0 * EPS * 13.005021117466894651792463195,
        "AggQ delta: got {v}"
    );
}

#[test]
fn agg_q_full_reg_normal_doubles() {
    let cases: &[(f64, f64)] = &[
        (0.1, 282.287364107970722480820996055),
        (0.5, -8.10459201840268733707015752686),
        (0.9, -69.4649911752863262667979987405),
    ];
    for &(x, ref_val) in cases {
        let v = AggQ::reg(AS, LM, NF, x);
        assert_near(
            v,
            ref_val,
            err_bound_near_one(ref_val, x),
            &format!("AggQ reg x={x}"),
        );
    }
}

#[test]
fn agg_q_full_plus_normal_doubles() {
    let cases: &[(f64, f64)] = &[
        (0.1, 46.9704828864620401234066180582),
        (0.5, 84.5468691956316722221319125047),
        (0.9, 422.734345978158361110659562524),
    ];
    for &(x, ref_val) in cases {
        let v = AggQ::plus(AS, LM, NF, x);
        assert_near(
            v,
            ref_val,
            err_bound_near_one(ref_val, x),
            &format!("AggQ plus x={x}"),
        );
    }
}

#[test]
fn agg_q_trunc_as1_delta() {
    let v = AggQ::delta_trunc_as(1, AS, LM, NF);
    let ref_val = 0.166666666666666666666666666667;
    assert!(
        (v - ref_val).abs() <= 4.0 * EPS * ref_val.abs(),
        "AggQ trunc_as1 delta: got {v}"
    );
}

// ═══════════════════ AQqPS (reg-only) ═══════════════════

#[test]
fn a_qq_ps_full_reg_powers_of_4() {
    let cases: &[(f64, f64)] = &[
        (
            9.31322574615478515625e-10,
            2.71385701515686759164116171416e11,
        ),
        (0.0625, -442.145967140465503706720765883),
        (0.9375, -4.42629021000634080573005433033),
    ];
    for &(x, ref_val) in cases {
        let v = AQqPS::reg(AS, LM, NF, x);
        assert_near(v, ref_val, err_bound(ref_val), &format!("AQqPS reg x={x}"));
    }
}

#[test]
fn a_qq_ps_full_reg_normal_doubles() {
    let cases: &[(f64, f64)] = &[
        (0.1, -308.964737327568383465605933051),
        (0.5, -53.4111590352759380297240652135),
        (0.9, -7.55235656007184082201368892365),
    ];
    for &(x, ref_val) in cases {
        let v = AQqPS::reg(AS, LM, NF, x);
        assert_near(
            v,
            ref_val,
            err_bound_near_one(ref_val, x),
            &format!("AQqPS reg x={x}"),
        );
    }
}

#[test]
fn a_qq_ps_trunc_as2_reg() {
    let cases: &[(f64, f64)] = &[
        (0.1, -22.8930872297648058883893355704),
        (0.5, -2.91734359098792785988671321303),
    ];
    for &(x, ref_val) in cases {
        let v = AQqPS::reg_trunc_as(2, AS, LM, NF, x);
        assert_near(
            v,
            ref_val,
            err_bound_near_one(ref_val, x),
            &format!("AQqPS trunc_as2 reg x={x}"),
        );
    }
}

// ═══════════════════ AQqPSs (reg-only) ═══════════════════

#[test]
fn a_qq_ps_s_full_reg() {
    let v = AQqPSs::reg(AS, LM, NF, 0.5);
    assert!(v.is_finite(), "AQqPSs reg(0.5) not finite: {v}");
}

// ═══════════════════ AqqQPS (reg-only) ═══════════════════

#[test]
fn aqq_q_ps_full_reg() {
    let v = AqqQPS::reg(AS, LM, NF, 0.5);
    assert!(v.is_finite(), "AqqQPS reg(0.5) not finite: {v}");
}

// ═══════════════════ AqgQ (reg-only) ═══════════════════

#[test]
fn aqg_q_full_reg() {
    let v = AqgQ::reg(AS, LM, NF, 0.5);
    assert!(v.is_finite(), "AqgQ reg(0.5) not finite: {v}");
}

// ═══════════════════ AgqQ (reg-only) ═══════════════════

#[test]
fn agq_q_full_reg() {
    let v = AgqQ::reg(AS, LM, NF, 0.5);
    assert!(v.is_finite(), "AgqQ reg(0.5) not finite: {v}");
}

// ═══════════════════ AQg (reg-only) ═══════════════════

#[test]
fn a_qg_full_reg() {
    let v = AQg::reg(AS, LM, NF, 0.5);
    assert!(v.is_finite(), "AQg reg(0.5) not finite: {v}");
}

// ═══════════════════ Polarized: PolAqqQNSEven ═══════════════════

#[test]
fn pol_aqq_q_ns_even_full_delta() {
    let v = PolAqqQNSEven::delta(AS, LM, NF);
    assert!(v.is_finite(), "PolAqqQNSEven delta not finite: {v}");
}

#[test]
fn pol_aqq_q_ns_even_full_reg() {
    let v = PolAqqQNSEven::reg(AS, LM, NF, 0.5);
    assert!(v.is_finite(), "PolAqqQNSEven reg(0.5) not finite: {v}");
}

#[test]
fn pol_aqq_q_ns_even_full_plus() {
    let v = PolAqqQNSEven::plus(AS, LM, NF, 0.5);
    assert!(v.is_finite(), "PolAqqQNSEven plus(0.5) not finite: {v}");
}

// ═══════════════════ Polarized: PolAqqQNSOdd ═══════════════════

#[test]
fn pol_aqq_q_ns_odd_full_delta() {
    let v = PolAqqQNSOdd::delta(AS, LM, NF);
    assert!(v.is_finite(), "PolAqqQNSOdd delta not finite: {v}");
}

#[test]
fn pol_aqq_q_ns_odd_full_reg() {
    let v = PolAqqQNSOdd::reg(AS, LM, NF, 0.5);
    assert!(v.is_finite(), "PolAqqQNSOdd reg(0.5) not finite: {v}");
}

#[test]
fn pol_aqq_q_ns_odd_full_plus() {
    let v = PolAqqQNSOdd::plus(AS, LM, NF, 0.5);
    assert!(v.is_finite(), "PolAqqQNSOdd plus(0.5) not finite: {v}");
}

// ═══════════════════ Polarized: PolAggQ ═══════════════════

#[test]
fn pol_agg_q_full_delta() {
    let v = PolAggQ::delta(AS, LM, NF);
    assert!(v.is_finite(), "PolAggQ delta not finite: {v}");
}

#[test]
fn pol_agg_q_full_reg() {
    let v = PolAggQ::reg(AS, LM, NF, 0.5);
    assert!(v.is_finite(), "PolAggQ reg(0.5) not finite: {v}");
}

#[test]
fn pol_agg_q_full_plus() {
    let v = PolAggQ::plus(AS, LM, NF, 0.5);
    assert!(v.is_finite(), "PolAggQ plus(0.5) not finite: {v}");
}

// ═══════════════════ Polarized reg-only ═══════════════════

#[test]
fn pol_a_qq_ps_full_reg() {
    let v = PolAQqPS::reg(AS, LM, NF, 0.5);
    assert!(v.is_finite(), "PolAQqPS reg(0.5) not finite: {v}");
}

#[test]
fn pol_a_qq_ps_s_full_reg() {
    let v = PolAQqPSs::reg(AS, LM, NF, 0.5);
    assert!(v.is_finite(), "PolAQqPSs reg(0.5) not finite: {v}");
}

#[test]
fn pol_aqq_q_ps_full_reg() {
    let v = PolAqqQPS::reg(AS, LM, NF, 0.5);
    assert!(v.is_finite(), "PolAqqQPS reg(0.5) not finite: {v}");
}

#[test]
fn pol_aqg_q_full_reg() {
    let v = PolAqgQ::reg(AS, LM, NF, 0.5);
    assert!(v.is_finite(), "PolAqgQ reg(0.5) not finite: {v}");
}

#[test]
fn pol_agq_q_full_reg() {
    let v = PolAgqQ::reg(AS, LM, NF, 0.5);
    assert!(v.is_finite(), "PolAgqQ reg(0.5) not finite: {v}");
}

#[test]
fn pol_a_qg_full_reg() {
    let v = PolAQg::reg(AS, LM, NF, 0.5);
    assert!(v.is_finite(), "PolAQg reg(0.5) not finite: {v}");
}

// ═══════════════════ Power range consistency ═══════════════════

#[test]
fn power_ranges_consistent() {
    let as_min = AggQ::reg_min_power();
    let as_max = AggQ::reg_max_power();
    assert!(as_min <= as_max);

    for order_as in as_min..=as_max {
        let lm_min = AggQ::reg_coeff_as_min_power(order_as);
        let lm_max = AggQ::reg_coeff_as_max_power(order_as);
        assert!(lm_min <= lm_max, "LM range invalid at as={order_as}");

        for order_lm in lm_min..=lm_max {
            let nf_min = AggQ::reg_coeff_as_lm_min_power(order_as, order_lm);
            let nf_max = AggQ::reg_coeff_as_lm_max_power(order_as, order_lm);
            assert!(
                nf_min <= nf_max,
                "NF range invalid at as={order_as}, LM={order_lm}"
            );
        }
    }
}

// ═══════════════════ Coefficient access ═══════════════════

#[test]
fn coeff_access_finite() {
    let min = AqqQNSEven::reg_min_power();
    let v = AqqQNSEven::reg_coeff_as(min, LM, NF, 0.5);
    assert!(
        v.is_finite(),
        "AqqQNSEven reg_coeff_as({min}) not finite: {v}"
    );

    let lm_min = AqqQNSEven::reg_coeff_as_min_power(min);
    let v2 = AqqQNSEven::reg_coeff_as_lm(min, lm_min, NF, 0.5);
    assert!(
        v2.is_finite(),
        "AqqQNSEven reg_coeff_as_lm not finite: {v2}"
    );

    let nf_min = AqqQNSEven::reg_coeff_as_lm_min_power(min, lm_min);
    let v3 = AqqQNSEven::reg_coeff_as_lm_nf(min, lm_min, nf_min, 0.5);
    assert!(
        v3.is_finite(),
        "AqqQNSEven reg_coeff_as_lm_nf not finite: {v3}"
    );
}
