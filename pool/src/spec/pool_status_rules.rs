use cvlr::clog;
use soroban_sdk::Env;

use cvlr::asserts::{cvlr_assert, cvlr_assume};
use cvlr_soroban_derive::rule;

use crate::pool::execute_update_pool_status;
use crate::spec::{GHOST_MET_THRESHOLD, GHOST_POOL_BACKSTOP_DATA};
use crate::{storage, PoolConfig};

// after update status, the status can only be 1, 3, 5
#[rule]
pub fn verify_status_update(e: Env) {
    let pool_config: PoolConfig = cvlr::nondet();

    storage::set_pool_config(&e, &pool_config);

    let status_after = execute_update_pool_status(&e);

    clog!(status_after);

    cvlr_assert!(status_after == 1 || status_after == 3 || status_after == 5);
}

#[rule]
pub fn verify_update_status_6(e: Env) {
    let pool_config: PoolConfig = cvlr::nondet();

    cvlr_assume!(pool_config.status == 6);

    storage::set_pool_config(&e, &pool_config);

    execute_update_pool_status(&e);

    cvlr_assert!(false); // should not reach due to panic
}

#[rule]
pub fn verify_update_status_4(e: Env) {
    let pool_config: PoolConfig = cvlr::nondet();

    cvlr_assume!(pool_config.status == 4);

    storage::set_pool_config(&e, &pool_config);

    execute_update_pool_status(&e);

    cvlr_assert!(false); // should not reach due to panic
}

#[rule]
pub fn verify_update_status_2(e: Env) {
    let pool_config: PoolConfig = cvlr::nondet();

    cvlr_assume!(pool_config.status == 2);

    storage::set_pool_config(&e, &pool_config);

    let status_after = execute_update_pool_status(&e);

    cvlr_assert!(unsafe { GHOST_POOL_BACKSTOP_DATA.q4w_pct < 0_7500000 } || status_after == 5);
}

#[rule]
pub fn verify_update_status_0_a(e: Env) {
    let pool_config: PoolConfig = cvlr::nondet();

    cvlr_assume!(pool_config.status == 0);

    storage::set_pool_config(&e, &pool_config);

    let status_after = execute_update_pool_status(&e);

    cvlr_assume!(unsafe {GHOST_POOL_BACKSTOP_DATA.q4w_pct >= 0_5000000} || unsafe { !GHOST_MET_THRESHOLD });

    cvlr_assert!(status_after == 3);
}

/**
 * NOTE that this rule is directly taken from
 * the official documentation: https://docs.blend.capital/tech-docs/core-contracts/lending-pool/pool-management#permissionless-updates
 * However, as you can see below,
 * the documentation does not make sense because it is not possible
 * for GHOST_POOL_BACKSTOP_DATA.q4w_pct < 0_5000000 and also >= 0_6000000.
 * This rule will therefore always pass but it is vacuous.
 * Please consider updating the official documentation.
 */
#[rule]
pub fn verify_update_status_0_b(e: Env) {
    let pool_config: PoolConfig = cvlr::nondet();

    cvlr_assume!(pool_config.status == 0);

    storage::set_pool_config(&e, &pool_config);

    let status_after = execute_update_pool_status(&e);

    cvlr_assume!(!(unsafe {GHOST_POOL_BACKSTOP_DATA.q4w_pct >= 0_5000000} || unsafe { !GHOST_MET_THRESHOLD }));
    cvlr_assume!(unsafe {GHOST_POOL_BACKSTOP_DATA.q4w_pct >= 0_6000000});

    cvlr_assert!(status_after == 5);
}

#[rule]
pub fn verify_update_status_0_c(e: Env) {
    let pool_config: PoolConfig = cvlr::nondet();

    cvlr_assume!(pool_config.status == 0);

    storage::set_pool_config(&e, &pool_config);

    let status_after = execute_update_pool_status(&e);

    cvlr_assume!(!(unsafe {GHOST_POOL_BACKSTOP_DATA.q4w_pct >= 0_5000000} || unsafe { !GHOST_MET_THRESHOLD }));
    cvlr_assume!(!unsafe {GHOST_POOL_BACKSTOP_DATA.q4w_pct >= 0_6000000});
    cvlr_assume!(unsafe {GHOST_POOL_BACKSTOP_DATA.q4w_pct >= 0_3000000} || unsafe { !GHOST_MET_THRESHOLD });

    cvlr_assert!(status_after == 3);
}

#[rule]
pub fn verify_update_status_0_d(e: Env) {
    let pool_config: PoolConfig = cvlr::nondet();

    cvlr_assume!(pool_config.status == 0);

    storage::set_pool_config(&e, &pool_config);

    let status_after = execute_update_pool_status(&e);

    cvlr_assume!(!(unsafe {GHOST_POOL_BACKSTOP_DATA.q4w_pct >= 0_5000000} || unsafe { !GHOST_MET_THRESHOLD }));
    cvlr_assume!(!unsafe {GHOST_POOL_BACKSTOP_DATA.q4w_pct >= 0_6000000});
    cvlr_assume!(!(unsafe {GHOST_POOL_BACKSTOP_DATA.q4w_pct >= 0_3000000} || unsafe { !GHOST_MET_THRESHOLD }));

    cvlr_assert!(status_after == 1);
}

#[rule]
pub fn verify_update_status_other_a(e: Env) {
    let pool_config: PoolConfig = cvlr::nondet();

    cvlr_assume!(
        pool_config.status != 0
            && pool_config.status != 2
            && pool_config.status != 4
            && pool_config.status != 6
    );

    storage::set_pool_config(&e, &pool_config);

    let status_after = execute_update_pool_status(&e);

    cvlr_assume!(unsafe { GHOST_POOL_BACKSTOP_DATA.q4w_pct >= 0_6000000 });

    cvlr_assert!(status_after == 5);
}

#[rule]
pub fn verify_update_status_other_b(e: Env) {
    let pool_config: PoolConfig = cvlr::nondet();

    cvlr_assume!(
        pool_config.status != 0
            && pool_config.status != 2
            && pool_config.status != 4
            && pool_config.status != 6
    );

    storage::set_pool_config(&e, &pool_config);

    let status_after = execute_update_pool_status(&e);

    cvlr_assume!(!unsafe { GHOST_POOL_BACKSTOP_DATA.q4w_pct >= 0_6000000 });
    cvlr_assume!(unsafe {GHOST_POOL_BACKSTOP_DATA.q4w_pct >= 0_3000000} || unsafe { !GHOST_MET_THRESHOLD });

    cvlr_assert!(status_after == 3);
}

#[rule]
pub fn verify_update_status_other_c(e: Env) {
    let pool_config: PoolConfig = cvlr::nondet();

    cvlr_assume!(
        pool_config.status != 0
            && pool_config.status != 2
            && pool_config.status != 4
            && pool_config.status != 6
    );

    storage::set_pool_config(&e, &pool_config);

    let status_after = execute_update_pool_status(&e);

    cvlr_assume!(!unsafe { GHOST_POOL_BACKSTOP_DATA.q4w_pct >= 0_6000000 });
    cvlr_assume!(!(unsafe {GHOST_POOL_BACKSTOP_DATA.q4w_pct >= 0_3000000} || unsafe { !GHOST_MET_THRESHOLD }));

    cvlr_assert!(status_after == 1);
}
