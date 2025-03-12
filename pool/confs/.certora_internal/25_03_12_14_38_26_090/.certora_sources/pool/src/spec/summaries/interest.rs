use cast::i128;
use soroban_fixed_point_math::SorobanFixedPoint;
use soroban_sdk::{panic_with_error, Env};

use crate::{
    constants::{SCALAR_12, SCALAR_7, SECONDS_PER_YEAR},
    storage::ReserveConfig,
    PoolError,
};

/** Summarize calc_accrual from pool/src/pool/interest.rs
 * by separating the cur_ir computation out to a separate function
 * which is then called within the calc_accural_summary function below.
 * Note that this is NOT sound but here we don't care about the actual value of cur_ir,
 * we just want to know if the second branch will ever be taken.
*/
pub fn calc_ir(
    e: &Env,
    config: &ReserveConfig,
    cur_util: i128,
    ir_mod: i128,
    last_time: u64,
) -> i128 {
    let cur_ir: i128;
    let target_util: i128 = i128(config.util);
    if cur_util <= target_util {
        cur_ir = 1;
    } else if cur_util <= 0_9500000 {
        cur_ir = 2;
    } else {
        cur_ir = 3;
    }
    return cur_ir
}

// Not used right now.
pub fn calc_accrual_summary(e: &Env,
    config: &ReserveConfig,
    cur_util: i128,
    ir_mod: i128,
    last_time: u64,
) -> (i128, i128) {
    let target_util: i128 = i128(config.util);
    let cur_ir = calc_ir(e, config, cur_util, ir_mod, last_time);

    // update rate_modifier 
    let delta_time = i128(e.ledger().timestamp() - last_time);
    // this should never occur, but require some time to pass
    if delta_time < 1 {
        panic_with_error!(e, PoolError::InternalError);
    }
    // util dif 7 decimals
    let util_dif = cur_util - target_util;
    let new_ir_mod: i128;
    if util_dif >= 0 {
        // rate modifier increasing
        let util_error = delta_time * util_dif;
        let rate_dif = util_error.fixed_mul_floor(e, &i128(config.reactivity), &SCALAR_7);
        let next_ir_mod = ir_mod + rate_dif;
        let ir_mod_max = 10 * SCALAR_7;
        if next_ir_mod > ir_mod_max {
            new_ir_mod = ir_mod_max;
        } else {
            new_ir_mod = next_ir_mod;
        }
    } else {
        // rate modifier decreasing
        let util_error = delta_time * util_dif;
        let rate_dif = util_error.fixed_mul_ceil(e, &i128(config.reactivity), &SCALAR_7);
        let next_ir_mod = ir_mod + rate_dif;
        let ir_mod_min = SCALAR_7 / 10;
        if next_ir_mod < ir_mod_min {
            new_ir_mod = ir_mod_min;
        } else {
            new_ir_mod = next_ir_mod;
        }
    }

    // calc accrual amount over blocks
    // scale delta_time to 12 decimals so time_weight is scaled to 12 decimals
    let delta_time_scaled = delta_time * SCALAR_12;
    let time_weight = delta_time_scaled / SECONDS_PER_YEAR;
    (
        // accrual scaled to 12 decimals
        SCALAR_12 + time_weight.fixed_mul_ceil(e, &cur_ir, &SCALAR_7),
        new_ir_mod,
    )
}