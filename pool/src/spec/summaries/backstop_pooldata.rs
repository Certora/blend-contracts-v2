
use crate::{dependencies::PoolBackstopData, spec::GHOST_POOL_BACKSTOP_DATA};

pub fn pool_data_summary() -> PoolBackstopData {
    let pool_data = PoolBackstopData {
        tokens: cvlr::nondet(),
        q4w_pct: cvlr::nondet(),
        blnd: cvlr::nondet(),
        usdc: cvlr::nondet(),
    };
    unsafe { GHOST_POOL_BACKSTOP_DATA = pool_data.clone() };
    return pool_data
}