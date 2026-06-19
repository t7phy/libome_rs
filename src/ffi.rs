// Raw FFI bindings to the libome C interface.
//
// Every OME exposes a "reg" (regular) part. Some also expose "plus" and "delta"
// parts. Each part has evaluation functions and range-getter functions.

unsafe extern "C" {
    // ───────────────────────── AqqQNSEven ─────────────────────────
    pub fn ome_AqqQNSEven_reg(as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AqqQNSEven_reg_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64, x: f64)
    -> f64;
    pub fn ome_AqqQNSEven_reg_coeff_as(order_as: i32, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AqqQNSEven_reg_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64, x: f64) -> f64;
    pub fn ome_AqqQNSEven_reg_coeff_as_LM_NF(
        order_as: i32,
        order_lm: i32,
        order_nf: i32,
        x: f64,
    ) -> f64;
    pub fn ome_AqqQNSEven_reg_min_power() -> i32;
    pub fn ome_AqqQNSEven_reg_max_power() -> i32;
    pub fn ome_AqqQNSEven_reg_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_AqqQNSEven_reg_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_AqqQNSEven_reg_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_AqqQNSEven_reg_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    pub fn ome_AqqQNSEven_plus(as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AqqQNSEven_plus_trunc_as(
        trunc_order: i32,
        as_: f64,
        lm: f64,
        nf: f64,
        x: f64,
    ) -> f64;
    pub fn ome_AqqQNSEven_plus_coeff_as(order_as: i32, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AqqQNSEven_plus_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64, x: f64) -> f64;
    pub fn ome_AqqQNSEven_plus_coeff_as_LM_NF(
        order_as: i32,
        order_lm: i32,
        order_nf: i32,
        x: f64,
    ) -> f64;
    pub fn ome_AqqQNSEven_plus_min_power() -> i32;
    pub fn ome_AqqQNSEven_plus_max_power() -> i32;
    pub fn ome_AqqQNSEven_plus_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_AqqQNSEven_plus_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_AqqQNSEven_plus_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_AqqQNSEven_plus_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    pub fn ome_AqqQNSEven_delta(as_: f64, lm: f64, nf: f64) -> f64;
    pub fn ome_AqqQNSEven_delta_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64) -> f64;
    pub fn ome_AqqQNSEven_delta_coeff_as(order_as: i32, lm: f64, nf: f64) -> f64;
    pub fn ome_AqqQNSEven_delta_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64) -> f64;
    pub fn ome_AqqQNSEven_delta_coeff_as_LM_NF(order_as: i32, order_lm: i32, order_nf: i32) -> f64;
    pub fn ome_AqqQNSEven_delta_min_power() -> i32;
    pub fn ome_AqqQNSEven_delta_max_power() -> i32;
    pub fn ome_AqqQNSEven_delta_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_AqqQNSEven_delta_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_AqqQNSEven_delta_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_AqqQNSEven_delta_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    // ───────────────────────── AqqQNSOdd ──────────────────────────
    pub fn ome_AqqQNSOdd_reg(as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AqqQNSOdd_reg_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AqqQNSOdd_reg_coeff_as(order_as: i32, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AqqQNSOdd_reg_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64, x: f64) -> f64;
    pub fn ome_AqqQNSOdd_reg_coeff_as_LM_NF(
        order_as: i32,
        order_lm: i32,
        order_nf: i32,
        x: f64,
    ) -> f64;
    pub fn ome_AqqQNSOdd_reg_min_power() -> i32;
    pub fn ome_AqqQNSOdd_reg_max_power() -> i32;
    pub fn ome_AqqQNSOdd_reg_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_AqqQNSOdd_reg_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_AqqQNSOdd_reg_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_AqqQNSOdd_reg_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    pub fn ome_AqqQNSOdd_plus(as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AqqQNSOdd_plus_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64, x: f64)
    -> f64;
    pub fn ome_AqqQNSOdd_plus_coeff_as(order_as: i32, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AqqQNSOdd_plus_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64, x: f64) -> f64;
    pub fn ome_AqqQNSOdd_plus_coeff_as_LM_NF(
        order_as: i32,
        order_lm: i32,
        order_nf: i32,
        x: f64,
    ) -> f64;
    pub fn ome_AqqQNSOdd_plus_min_power() -> i32;
    pub fn ome_AqqQNSOdd_plus_max_power() -> i32;
    pub fn ome_AqqQNSOdd_plus_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_AqqQNSOdd_plus_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_AqqQNSOdd_plus_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_AqqQNSOdd_plus_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    pub fn ome_AqqQNSOdd_delta(as_: f64, lm: f64, nf: f64) -> f64;
    pub fn ome_AqqQNSOdd_delta_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64) -> f64;
    pub fn ome_AqqQNSOdd_delta_coeff_as(order_as: i32, lm: f64, nf: f64) -> f64;
    pub fn ome_AqqQNSOdd_delta_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64) -> f64;
    pub fn ome_AqqQNSOdd_delta_coeff_as_LM_NF(order_as: i32, order_lm: i32, order_nf: i32) -> f64;
    pub fn ome_AqqQNSOdd_delta_min_power() -> i32;
    pub fn ome_AqqQNSOdd_delta_max_power() -> i32;
    pub fn ome_AqqQNSOdd_delta_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_AqqQNSOdd_delta_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_AqqQNSOdd_delta_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_AqqQNSOdd_delta_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    // ───────────────────────── AggQ ───────────────────────────────
    pub fn ome_AggQ_reg(as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AggQ_reg_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AggQ_reg_coeff_as(order_as: i32, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AggQ_reg_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64, x: f64) -> f64;
    pub fn ome_AggQ_reg_coeff_as_LM_NF(order_as: i32, order_lm: i32, order_nf: i32, x: f64) -> f64;
    pub fn ome_AggQ_reg_min_power() -> i32;
    pub fn ome_AggQ_reg_max_power() -> i32;
    pub fn ome_AggQ_reg_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_AggQ_reg_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_AggQ_reg_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_AggQ_reg_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    pub fn ome_AggQ_plus(as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AggQ_plus_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AggQ_plus_coeff_as(order_as: i32, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AggQ_plus_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64, x: f64) -> f64;
    pub fn ome_AggQ_plus_coeff_as_LM_NF(order_as: i32, order_lm: i32, order_nf: i32, x: f64)
    -> f64;
    pub fn ome_AggQ_plus_min_power() -> i32;
    pub fn ome_AggQ_plus_max_power() -> i32;
    pub fn ome_AggQ_plus_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_AggQ_plus_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_AggQ_plus_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_AggQ_plus_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    pub fn ome_AggQ_delta(as_: f64, lm: f64, nf: f64) -> f64;
    pub fn ome_AggQ_delta_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64) -> f64;
    pub fn ome_AggQ_delta_coeff_as(order_as: i32, lm: f64, nf: f64) -> f64;
    pub fn ome_AggQ_delta_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64) -> f64;
    pub fn ome_AggQ_delta_coeff_as_LM_NF(order_as: i32, order_lm: i32, order_nf: i32) -> f64;
    pub fn ome_AggQ_delta_min_power() -> i32;
    pub fn ome_AggQ_delta_max_power() -> i32;
    pub fn ome_AggQ_delta_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_AggQ_delta_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_AggQ_delta_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_AggQ_delta_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    // ───────────────────────── AQqPS ──────────────────────────────
    pub fn ome_AQqPS_reg(as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AQqPS_reg_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AQqPS_reg_coeff_as(order_as: i32, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AQqPS_reg_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64, x: f64) -> f64;
    pub fn ome_AQqPS_reg_coeff_as_LM_NF(order_as: i32, order_lm: i32, order_nf: i32, x: f64)
    -> f64;
    pub fn ome_AQqPS_reg_min_power() -> i32;
    pub fn ome_AQqPS_reg_max_power() -> i32;
    pub fn ome_AQqPS_reg_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_AQqPS_reg_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_AQqPS_reg_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_AQqPS_reg_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    // ───────────────────────── AQqPSs ─────────────────────────────
    pub fn ome_AQqPSs_reg(as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AQqPSs_reg_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AQqPSs_reg_coeff_as(order_as: i32, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AQqPSs_reg_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64, x: f64) -> f64;
    pub fn ome_AQqPSs_reg_coeff_as_LM_NF(
        order_as: i32,
        order_lm: i32,
        order_nf: i32,
        x: f64,
    ) -> f64;
    pub fn ome_AQqPSs_reg_min_power() -> i32;
    pub fn ome_AQqPSs_reg_max_power() -> i32;
    pub fn ome_AQqPSs_reg_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_AQqPSs_reg_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_AQqPSs_reg_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_AQqPSs_reg_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    // ───────────────────────── AqqQPS ─────────────────────────────
    pub fn ome_AqqQPS_reg(as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AqqQPS_reg_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AqqQPS_reg_coeff_as(order_as: i32, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AqqQPS_reg_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64, x: f64) -> f64;
    pub fn ome_AqqQPS_reg_coeff_as_LM_NF(
        order_as: i32,
        order_lm: i32,
        order_nf: i32,
        x: f64,
    ) -> f64;
    pub fn ome_AqqQPS_reg_min_power() -> i32;
    pub fn ome_AqqQPS_reg_max_power() -> i32;
    pub fn ome_AqqQPS_reg_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_AqqQPS_reg_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_AqqQPS_reg_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_AqqQPS_reg_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    // ───────────────────────── AqgQ ───────────────────────────────
    pub fn ome_AqgQ_reg(as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AqgQ_reg_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AqgQ_reg_coeff_as(order_as: i32, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AqgQ_reg_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64, x: f64) -> f64;
    pub fn ome_AqgQ_reg_coeff_as_LM_NF(order_as: i32, order_lm: i32, order_nf: i32, x: f64) -> f64;
    pub fn ome_AqgQ_reg_min_power() -> i32;
    pub fn ome_AqgQ_reg_max_power() -> i32;
    pub fn ome_AqgQ_reg_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_AqgQ_reg_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_AqgQ_reg_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_AqgQ_reg_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    // ───────────────────────── AgqQ ───────────────────────────────
    pub fn ome_AgqQ_reg(as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AgqQ_reg_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AgqQ_reg_coeff_as(order_as: i32, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AgqQ_reg_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64, x: f64) -> f64;
    pub fn ome_AgqQ_reg_coeff_as_LM_NF(order_as: i32, order_lm: i32, order_nf: i32, x: f64) -> f64;
    pub fn ome_AgqQ_reg_min_power() -> i32;
    pub fn ome_AgqQ_reg_max_power() -> i32;
    pub fn ome_AgqQ_reg_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_AgqQ_reg_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_AgqQ_reg_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_AgqQ_reg_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    // ───────────────────────── AQg ────────────────────────────────
    pub fn ome_AQg_reg(as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AQg_reg_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AQg_reg_coeff_as(order_as: i32, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_AQg_reg_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64, x: f64) -> f64;
    pub fn ome_AQg_reg_coeff_as_LM_NF(order_as: i32, order_lm: i32, order_nf: i32, x: f64) -> f64;
    pub fn ome_AQg_reg_min_power() -> i32;
    pub fn ome_AQg_reg_max_power() -> i32;
    pub fn ome_AQg_reg_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_AQg_reg_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_AQg_reg_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_AQg_reg_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    // ───────────────────────── polAqqQNSEven ──────────────────────
    pub fn ome_polAqqQNSEven_reg(as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAqqQNSEven_reg_trunc_as(
        trunc_order: i32,
        as_: f64,
        lm: f64,
        nf: f64,
        x: f64,
    ) -> f64;
    pub fn ome_polAqqQNSEven_reg_coeff_as(order_as: i32, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAqqQNSEven_reg_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64, x: f64) -> f64;
    pub fn ome_polAqqQNSEven_reg_coeff_as_LM_NF(
        order_as: i32,
        order_lm: i32,
        order_nf: i32,
        x: f64,
    ) -> f64;
    pub fn ome_polAqqQNSEven_reg_min_power() -> i32;
    pub fn ome_polAqqQNSEven_reg_max_power() -> i32;
    pub fn ome_polAqqQNSEven_reg_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_polAqqQNSEven_reg_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_polAqqQNSEven_reg_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_polAqqQNSEven_reg_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    pub fn ome_polAqqQNSEven_plus(as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAqqQNSEven_plus_trunc_as(
        trunc_order: i32,
        as_: f64,
        lm: f64,
        nf: f64,
        x: f64,
    ) -> f64;
    pub fn ome_polAqqQNSEven_plus_coeff_as(order_as: i32, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAqqQNSEven_plus_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64, x: f64)
    -> f64;
    pub fn ome_polAqqQNSEven_plus_coeff_as_LM_NF(
        order_as: i32,
        order_lm: i32,
        order_nf: i32,
        x: f64,
    ) -> f64;
    pub fn ome_polAqqQNSEven_plus_min_power() -> i32;
    pub fn ome_polAqqQNSEven_plus_max_power() -> i32;
    pub fn ome_polAqqQNSEven_plus_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_polAqqQNSEven_plus_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_polAqqQNSEven_plus_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_polAqqQNSEven_plus_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    pub fn ome_polAqqQNSEven_delta(as_: f64, lm: f64, nf: f64) -> f64;
    pub fn ome_polAqqQNSEven_delta_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64) -> f64;
    pub fn ome_polAqqQNSEven_delta_coeff_as(order_as: i32, lm: f64, nf: f64) -> f64;
    pub fn ome_polAqqQNSEven_delta_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64) -> f64;
    pub fn ome_polAqqQNSEven_delta_coeff_as_LM_NF(
        order_as: i32,
        order_lm: i32,
        order_nf: i32,
    ) -> f64;
    pub fn ome_polAqqQNSEven_delta_min_power() -> i32;
    pub fn ome_polAqqQNSEven_delta_max_power() -> i32;
    pub fn ome_polAqqQNSEven_delta_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_polAqqQNSEven_delta_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_polAqqQNSEven_delta_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_polAqqQNSEven_delta_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    // ───────────────────────── polAqqQNSOdd ───────────────────────
    pub fn ome_polAqqQNSOdd_reg(as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAqqQNSOdd_reg_trunc_as(
        trunc_order: i32,
        as_: f64,
        lm: f64,
        nf: f64,
        x: f64,
    ) -> f64;
    pub fn ome_polAqqQNSOdd_reg_coeff_as(order_as: i32, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAqqQNSOdd_reg_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64, x: f64) -> f64;
    pub fn ome_polAqqQNSOdd_reg_coeff_as_LM_NF(
        order_as: i32,
        order_lm: i32,
        order_nf: i32,
        x: f64,
    ) -> f64;
    pub fn ome_polAqqQNSOdd_reg_min_power() -> i32;
    pub fn ome_polAqqQNSOdd_reg_max_power() -> i32;
    pub fn ome_polAqqQNSOdd_reg_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_polAqqQNSOdd_reg_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_polAqqQNSOdd_reg_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_polAqqQNSOdd_reg_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    pub fn ome_polAqqQNSOdd_plus(as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAqqQNSOdd_plus_trunc_as(
        trunc_order: i32,
        as_: f64,
        lm: f64,
        nf: f64,
        x: f64,
    ) -> f64;
    pub fn ome_polAqqQNSOdd_plus_coeff_as(order_as: i32, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAqqQNSOdd_plus_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64, x: f64) -> f64;
    pub fn ome_polAqqQNSOdd_plus_coeff_as_LM_NF(
        order_as: i32,
        order_lm: i32,
        order_nf: i32,
        x: f64,
    ) -> f64;
    pub fn ome_polAqqQNSOdd_plus_min_power() -> i32;
    pub fn ome_polAqqQNSOdd_plus_max_power() -> i32;
    pub fn ome_polAqqQNSOdd_plus_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_polAqqQNSOdd_plus_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_polAqqQNSOdd_plus_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_polAqqQNSOdd_plus_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    pub fn ome_polAqqQNSOdd_delta(as_: f64, lm: f64, nf: f64) -> f64;
    pub fn ome_polAqqQNSOdd_delta_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64) -> f64;
    pub fn ome_polAqqQNSOdd_delta_coeff_as(order_as: i32, lm: f64, nf: f64) -> f64;
    pub fn ome_polAqqQNSOdd_delta_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64) -> f64;
    pub fn ome_polAqqQNSOdd_delta_coeff_as_LM_NF(
        order_as: i32,
        order_lm: i32,
        order_nf: i32,
    ) -> f64;
    pub fn ome_polAqqQNSOdd_delta_min_power() -> i32;
    pub fn ome_polAqqQNSOdd_delta_max_power() -> i32;
    pub fn ome_polAqqQNSOdd_delta_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_polAqqQNSOdd_delta_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_polAqqQNSOdd_delta_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_polAqqQNSOdd_delta_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    // ───────────────────────── polAggQ ────────────────────────────
    pub fn ome_polAggQ_reg(as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAggQ_reg_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAggQ_reg_coeff_as(order_as: i32, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAggQ_reg_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64, x: f64) -> f64;
    pub fn ome_polAggQ_reg_coeff_as_LM_NF(
        order_as: i32,
        order_lm: i32,
        order_nf: i32,
        x: f64,
    ) -> f64;
    pub fn ome_polAggQ_reg_min_power() -> i32;
    pub fn ome_polAggQ_reg_max_power() -> i32;
    pub fn ome_polAggQ_reg_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_polAggQ_reg_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_polAggQ_reg_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_polAggQ_reg_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    pub fn ome_polAggQ_plus(as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAggQ_plus_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAggQ_plus_coeff_as(order_as: i32, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAggQ_plus_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64, x: f64) -> f64;
    pub fn ome_polAggQ_plus_coeff_as_LM_NF(
        order_as: i32,
        order_lm: i32,
        order_nf: i32,
        x: f64,
    ) -> f64;
    pub fn ome_polAggQ_plus_min_power() -> i32;
    pub fn ome_polAggQ_plus_max_power() -> i32;
    pub fn ome_polAggQ_plus_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_polAggQ_plus_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_polAggQ_plus_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_polAggQ_plus_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    pub fn ome_polAggQ_delta(as_: f64, lm: f64, nf: f64) -> f64;
    pub fn ome_polAggQ_delta_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64) -> f64;
    pub fn ome_polAggQ_delta_coeff_as(order_as: i32, lm: f64, nf: f64) -> f64;
    pub fn ome_polAggQ_delta_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64) -> f64;
    pub fn ome_polAggQ_delta_coeff_as_LM_NF(order_as: i32, order_lm: i32, order_nf: i32) -> f64;
    pub fn ome_polAggQ_delta_min_power() -> i32;
    pub fn ome_polAggQ_delta_max_power() -> i32;
    pub fn ome_polAggQ_delta_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_polAggQ_delta_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_polAggQ_delta_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_polAggQ_delta_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    // ───────────────────────── polAQqPS ───────────────────────────
    pub fn ome_polAQqPS_reg(as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAQqPS_reg_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAQqPS_reg_coeff_as(order_as: i32, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAQqPS_reg_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64, x: f64) -> f64;
    pub fn ome_polAQqPS_reg_coeff_as_LM_NF(
        order_as: i32,
        order_lm: i32,
        order_nf: i32,
        x: f64,
    ) -> f64;
    pub fn ome_polAQqPS_reg_min_power() -> i32;
    pub fn ome_polAQqPS_reg_max_power() -> i32;
    pub fn ome_polAQqPS_reg_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_polAQqPS_reg_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_polAQqPS_reg_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_polAQqPS_reg_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    // ───────────────────────── polAQqPSs ──────────────────────────
    pub fn ome_polAQqPSs_reg(as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAQqPSs_reg_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAQqPSs_reg_coeff_as(order_as: i32, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAQqPSs_reg_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64, x: f64) -> f64;
    pub fn ome_polAQqPSs_reg_coeff_as_LM_NF(
        order_as: i32,
        order_lm: i32,
        order_nf: i32,
        x: f64,
    ) -> f64;
    pub fn ome_polAQqPSs_reg_min_power() -> i32;
    pub fn ome_polAQqPSs_reg_max_power() -> i32;
    pub fn ome_polAQqPSs_reg_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_polAQqPSs_reg_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_polAQqPSs_reg_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_polAQqPSs_reg_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    // ───────────────────────── polAqqQPS ──────────────────────────
    pub fn ome_polAqqQPS_reg(as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAqqQPS_reg_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAqqQPS_reg_coeff_as(order_as: i32, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAqqQPS_reg_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64, x: f64) -> f64;
    pub fn ome_polAqqQPS_reg_coeff_as_LM_NF(
        order_as: i32,
        order_lm: i32,
        order_nf: i32,
        x: f64,
    ) -> f64;
    pub fn ome_polAqqQPS_reg_min_power() -> i32;
    pub fn ome_polAqqQPS_reg_max_power() -> i32;
    pub fn ome_polAqqQPS_reg_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_polAqqQPS_reg_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_polAqqQPS_reg_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_polAqqQPS_reg_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    // ───────────────────────── polAqgQ ────────────────────────────
    pub fn ome_polAqgQ_reg(as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAqgQ_reg_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAqgQ_reg_coeff_as(order_as: i32, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAqgQ_reg_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64, x: f64) -> f64;
    pub fn ome_polAqgQ_reg_coeff_as_LM_NF(
        order_as: i32,
        order_lm: i32,
        order_nf: i32,
        x: f64,
    ) -> f64;
    pub fn ome_polAqgQ_reg_min_power() -> i32;
    pub fn ome_polAqgQ_reg_max_power() -> i32;
    pub fn ome_polAqgQ_reg_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_polAqgQ_reg_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_polAqgQ_reg_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_polAqgQ_reg_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    // ───────────────────────── polAgqQ ────────────────────────────
    pub fn ome_polAgqQ_reg(as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAgqQ_reg_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAgqQ_reg_coeff_as(order_as: i32, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAgqQ_reg_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64, x: f64) -> f64;
    pub fn ome_polAgqQ_reg_coeff_as_LM_NF(
        order_as: i32,
        order_lm: i32,
        order_nf: i32,
        x: f64,
    ) -> f64;
    pub fn ome_polAgqQ_reg_min_power() -> i32;
    pub fn ome_polAgqQ_reg_max_power() -> i32;
    pub fn ome_polAgqQ_reg_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_polAgqQ_reg_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_polAgqQ_reg_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_polAgqQ_reg_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;

    // ───────────────────────── polAQg ─────────────────────────────
    pub fn ome_polAQg_reg(as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAQg_reg_trunc_as(trunc_order: i32, as_: f64, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAQg_reg_coeff_as(order_as: i32, lm: f64, nf: f64, x: f64) -> f64;
    pub fn ome_polAQg_reg_coeff_as_LM(order_as: i32, order_lm: i32, nf: f64, x: f64) -> f64;
    pub fn ome_polAQg_reg_coeff_as_LM_NF(
        order_as: i32,
        order_lm: i32,
        order_nf: i32,
        x: f64,
    ) -> f64;
    pub fn ome_polAQg_reg_min_power() -> i32;
    pub fn ome_polAQg_reg_max_power() -> i32;
    pub fn ome_polAQg_reg_coeff_as_min_power(order_as: i32) -> i32;
    pub fn ome_polAQg_reg_coeff_as_max_power(order_as: i32) -> i32;
    pub fn ome_polAQg_reg_coeff_as_LM_min_power(order_as: i32, order_lm: i32) -> i32;
    pub fn ome_polAQg_reg_coeff_as_LM_max_power(order_as: i32, order_lm: i32) -> i32;
}

// ═══════════════════ GSL integration shim ═══════════════════

#[cfg(feature = "mellin")]
#[repr(C)]
pub struct OmeIntegrationResult {
    pub status: i32,
    pub result: f64,
    pub abs_error: f64,
}

#[cfg(feature = "mellin")]
unsafe extern "C" {
    pub fn ome_mellin_moment_AqqQNSEven(
        n: i32,
        as_: f64,
        lm: f64,
        nf: f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_moment_AqqQNSOdd(
        n: i32,
        as_: f64,
        lm: f64,
        nf: f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_moment_AggQ(
        n: i32,
        as_: f64,
        lm: f64,
        nf: f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_moment_AQqPS(
        n: i32,
        as_: f64,
        lm: f64,
        nf: f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_moment_AQqPSs(
        n: i32,
        as_: f64,
        lm: f64,
        nf: f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_moment_AqqQPS(
        n: i32,
        as_: f64,
        lm: f64,
        nf: f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_moment_AqgQ(
        n: i32,
        as_: f64,
        lm: f64,
        nf: f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_moment_AgqQ(
        n: i32,
        as_: f64,
        lm: f64,
        nf: f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_moment_AQg(
        n: i32,
        as_: f64,
        lm: f64,
        nf: f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_moment_polAqqQNSEven(
        n: i32,
        as_: f64,
        lm: f64,
        nf: f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_moment_polAqqQNSOdd(
        n: i32,
        as_: f64,
        lm: f64,
        nf: f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_moment_polAggQ(
        n: i32,
        as_: f64,
        lm: f64,
        nf: f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_moment_polAQqPS(
        n: i32,
        as_: f64,
        lm: f64,
        nf: f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_moment_polAQqPSs(
        n: i32,
        as_: f64,
        lm: f64,
        nf: f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_moment_polAqqQPS(
        n: i32,
        as_: f64,
        lm: f64,
        nf: f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_moment_polAqgQ(
        n: i32,
        as_: f64,
        lm: f64,
        nf: f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_moment_polAgqQ(
        n: i32,
        as_: f64,
        lm: f64,
        nf: f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_moment_polAQg(
        n: i32,
        as_: f64,
        lm: f64,
        nf: f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;

    pub fn ome_mellin_convolution_AqqQNSEven(
        x: f64,
        as_: f64,
        lm: f64,
        nf: f64,
        testfunc: unsafe extern "C" fn(f64) -> f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_convolution_AqqQNSOdd(
        x: f64,
        as_: f64,
        lm: f64,
        nf: f64,
        testfunc: unsafe extern "C" fn(f64) -> f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_convolution_AggQ(
        x: f64,
        as_: f64,
        lm: f64,
        nf: f64,
        testfunc: unsafe extern "C" fn(f64) -> f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_convolution_AQqPS(
        x: f64,
        as_: f64,
        lm: f64,
        nf: f64,
        testfunc: unsafe extern "C" fn(f64) -> f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_convolution_AQqPSs(
        x: f64,
        as_: f64,
        lm: f64,
        nf: f64,
        testfunc: unsafe extern "C" fn(f64) -> f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_convolution_AqqQPS(
        x: f64,
        as_: f64,
        lm: f64,
        nf: f64,
        testfunc: unsafe extern "C" fn(f64) -> f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_convolution_AqgQ(
        x: f64,
        as_: f64,
        lm: f64,
        nf: f64,
        testfunc: unsafe extern "C" fn(f64) -> f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_convolution_AgqQ(
        x: f64,
        as_: f64,
        lm: f64,
        nf: f64,
        testfunc: unsafe extern "C" fn(f64) -> f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_convolution_AQg(
        x: f64,
        as_: f64,
        lm: f64,
        nf: f64,
        testfunc: unsafe extern "C" fn(f64) -> f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_convolution_polAqqQNSEven(
        x: f64,
        as_: f64,
        lm: f64,
        nf: f64,
        testfunc: unsafe extern "C" fn(f64) -> f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_convolution_polAqqQNSOdd(
        x: f64,
        as_: f64,
        lm: f64,
        nf: f64,
        testfunc: unsafe extern "C" fn(f64) -> f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_convolution_polAggQ(
        x: f64,
        as_: f64,
        lm: f64,
        nf: f64,
        testfunc: unsafe extern "C" fn(f64) -> f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_convolution_polAQqPS(
        x: f64,
        as_: f64,
        lm: f64,
        nf: f64,
        testfunc: unsafe extern "C" fn(f64) -> f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_convolution_polAQqPSs(
        x: f64,
        as_: f64,
        lm: f64,
        nf: f64,
        testfunc: unsafe extern "C" fn(f64) -> f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_convolution_polAqqQPS(
        x: f64,
        as_: f64,
        lm: f64,
        nf: f64,
        testfunc: unsafe extern "C" fn(f64) -> f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_convolution_polAqgQ(
        x: f64,
        as_: f64,
        lm: f64,
        nf: f64,
        testfunc: unsafe extern "C" fn(f64) -> f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_convolution_polAgqQ(
        x: f64,
        as_: f64,
        lm: f64,
        nf: f64,
        testfunc: unsafe extern "C" fn(f64) -> f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
    pub fn ome_mellin_convolution_polAQg(
        x: f64,
        as_: f64,
        lm: f64,
        nf: f64,
        testfunc: unsafe extern "C" fn(f64) -> f64,
        eps_abs: f64,
        eps_rel: f64,
    ) -> OmeIntegrationResult;
}
