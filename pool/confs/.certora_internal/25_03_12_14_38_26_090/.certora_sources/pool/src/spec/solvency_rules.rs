use soroban_sdk::{Address, Env, Vec};

use soroban_fixed_point_math::SorobanFixedPoint;

use cvlr_soroban_derive::rule;
use cvlr::asserts::{cvlr_assert, cvlr_assume, cvlr_satisfy};
use cvlr::nondet::Nondet;
use cvlr_soroban::nondet_address;

use crate::pool::{Pool, User, Reserve};
use crate::pool::actions::Actions;
use crate::pool::actions::apply_supply::apply_supply;
use crate::PoolConfig;
use crate::constants::SCALAR_12;

/**
 * KEY PROPERTIES
- b_rate of a reserve does not decrease by some user_action:
at least for supply , supplyCollateral , withdraw , withdrawCollateral ,
borrow , repay , interest_accrual:

- b_rate can however decrease if defaulting 

- d_rate of a reserve does not decrease (the d_rate is totalLiabilities/dToken supply - e.g. when repay this is important, the value of 1 dToken should in the best case scenario increase, otherwise it’d mean that someone needs to pay back slightly less than they should which can compound to the protocol losing

 */


// gulp is the one functionality that affects b_rate so maybe we should show that it does not decrease b_rate?
// NOTE: I am not confident if this is the right way to go yet

// works
#[rule]
pub fn b_rate_not_decreased_by_gulp_when_nonpositive_accured(e: &Env) {
        let mut reserve: Reserve = cvlr::nondet();

        let pool_config: PoolConfig = cvlr::nondet();

        let b_rate_before = reserve.data.b_rate;

        let loan_accrual: i128 = cvlr::nondet();

        cvlr_assume!(loan_accrual <= 0);

        reserve.gulp(e, pool_config.bstop_rate, loan_accrual);
        
        let b_rate_after = reserve.data.b_rate;

        cvlr_assert!(b_rate_after == b_rate_before);
}

#[rule]
pub fn b_rate_not_decreased_by_gulp_when_positive_accured_zero_bstop_rate(e: &Env) {
        let mut reserve: Reserve = cvlr::nondet();

        let pool_config: PoolConfig = cvlr::nondet();

        let b_rate_before = reserve.data.b_rate;

        let loan_accrual: i128 = cvlr::nondet();

        cvlr_assume!(loan_accrual > 0 && pool_config.bstop_rate == 0);

        reserve.gulp(e, pool_config.bstop_rate, loan_accrual);
        
        let b_rate_after = reserve.data.b_rate;

        cvlr_assert!(b_rate_after >= b_rate_before);
}

/**
 * 
 * R76 = 1
 * R75 = 1
 * R188 = 0
 * R190 = 0
 * 
 * (R188%2^64)==R76 then
 *    (R190%2^64)>=R75
 * else
 *    (SignExtend(o1=0x7, o2=Mod(R188:bv256 0x10000000000000000), tag=bv256)) s≥ (SignExtend(o1=0x7, o2=R76:bv256, tag=bv256))
 * 
 * 
 */



// this gives spurious counter eg. but that is not surprising since everything is a nondet here.
#[rule]
pub fn b_rate_not_decreased_by_gulp_spurious(e: &Env) {
        let mut reserve: Reserve = cvlr::nondet();
        let pool_config: PoolConfig = cvlr::nondet();

        let b_rate_before = reserve.data.b_rate;

        let accrued = cvlr::nondet();

        cvlr_assume!(accrued > 0);

        reserve.gulp(e, pool_config.bstop_rate, accrued);

        let b_rate_after = reserve.data.b_rate;

        cvlr_assert!(b_rate_after >= b_rate_before);
}

#[rule]
pub fn b_supply_proportional_to_b_rate(e: &Env) {
        let mut reserve: Reserve = cvlr::nondet();
        let pool_config: PoolConfig = cvlr::nondet();

        let accrued: i128 = cvlr::nondet();
        let b_rate_before = reserve.data.b_rate;
        let b_supply_before = reserve.total_supply(e);

        reserve.gulp(e, pool_config.bstop_rate, accrued);

        let b_rate_after = reserve.data.b_rate;
        let b_supply_after = reserve.total_supply(e);

        cvlr_assume!(accrued > 0 && b_supply_after >= b_supply_before);
        cvlr_assert!(b_rate_after >= b_rate_before);
}


// Older Attempts
#[rule]
pub fn b_rate_not_decreased_by_gulp(e: &Env) {
        let mut reserve: Reserve = cvlr::nondet();
        let pool_config: PoolConfig = cvlr::nondet();

        let b_rate_before = reserve.data.b_rate;

        let loan_accrual: i128 = cvlr::nondet(); // see reserve.rs Line 74 for why this is needed

        let pre_update_liabilities = reserve.total_liabilities(e); // has muldiv in total_liabilities
        reserve.data.d_rate = loan_accrual.fixed_mul_ceil(e, &reserve.data.d_rate, &SCALAR_12); // more muldiv
        let accrued = reserve.total_liabilities(e) - pre_update_liabilities; // more muldiv in total_liabilities

        cvlr_assume!(accrued > 0);

        reserve.gulp(e, pool_config.bstop_rate, accrued);
        let b_rate_after = reserve.data.b_rate;

        cvlr_assert!(b_rate_after >= b_rate_before);
}