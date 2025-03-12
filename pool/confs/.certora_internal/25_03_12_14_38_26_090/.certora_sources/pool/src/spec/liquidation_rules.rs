use soroban_sdk::{Address, Env, Vec};

use cvlr_soroban_derive::rule;
use cvlr::asserts::{cvlr_assert, cvlr_assume, cvlr_satisfy};
use cvlr::nondet::Nondet;
use cvlr_soroban::nondet_address;

use crate::pool::{Pool, User, PositionData};
use crate::auctions::{
    backstop_interest_auction::{create_interest_auction_data, fill_interest_auction},
    bad_debt_auction::{create_bad_debt_auction_data, fill_bad_debt_auction},
    user_liquidation_auction::{create_user_liq_auction_data, fill_user_liq_auction},
};

// TIMED OUT EVEN WITH MANY PARTS in `create_user_liq_auction_data` COMMENTED OUT
// #[rule]
// pub fn liquidate_reverts_for_healthy_user(e: Env, bid: Vec<Address>, lot: Vec<Address>) {
//     let address = nondet_address();
//     let user = User::load(&e, &address);
//     let mut pool = Pool::load(&e);
//     let percent = cvlr::nondet();
//     create_user_liq_auction_data(&e, &address, &bid, &lot, percent);
//     unsafe {
//         cvlr_assert!(GHOST_PANIC_IF_HEALTHY_USER == true);
//     }
// }