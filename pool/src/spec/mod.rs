pub mod token;
pub(crate) mod summaries;
pub(crate) mod model;
pub(crate) mod health_rules;
pub(crate) mod user_rules;
pub(crate) mod interest_rules;
pub(crate) mod pool_status_rules;

#[macro_export]
macro_rules! nondet_expr {
    // nondet_expr(e) ==> nondet()
    ( $m:expr ) => {
        if cfg!(feature = "certora") {
            cvlr::nondet::nondet()
        } else {
            $m
        }
    };
    // nondet_expr(e1, e2; e)
    // { e1 = nondet(); e2 = nondet(); nondet() }
    ( $($e:expr ),* ; $m:expr ) => {
        if cfg!(feature = "certora") {
            $( $e = cvlr::nondet::nondet(); )* cvlr::nondet::nondet()
        } else {
            $m
        }
    }
}

use crate::dependencies::PoolBackstopData;

pub(crate) static mut GHOST_POOL_BACKSTOP_DATA: PoolBackstopData =
    PoolBackstopData {
        tokens: 0,
        q4w_pct: 0,
        blnd: 0,
        usdc: 0
    };

pub(crate) static mut GHOST_MET_THRESHOLD: bool = false;