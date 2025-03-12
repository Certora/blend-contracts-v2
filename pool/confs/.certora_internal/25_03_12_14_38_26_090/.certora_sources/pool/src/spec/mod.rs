// #[cfg(feature = "certora")]
pub mod token;

// #[cfg(feature = "certora")]
pub(crate) mod summaries;
// #[cfg(feature = "certora")]
pub(crate) mod model;
// #[cfg(feature = "certora")]
pub(crate) mod health_rules;
// #[cfg(feature = "certora")]
pub(crate) mod user_rules;
// #[cfg(feature = "certora")]
pub(crate) mod interest_rules;
// #[cfg(feature = "certora")]
pub(crate) mod solvency_rules;
// #[cfg(feature = "certora")]
pub(crate) mod pool_status_rules;
// #[cfg(feature = "certora")]
// pub(crate) mod liquidation_rules;


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