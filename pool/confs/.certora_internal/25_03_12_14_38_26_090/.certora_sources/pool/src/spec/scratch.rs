

#[contracttype]
pub struct T {
    collaterals: Map<u32, u32>,
    liabilities: Map<u32, u32>
}

#[rule]
pub fn map_update(e: &Env, u: u32, i: u32) {
    let mut m1: T = e.storage().persistent().get(&u).unwrap();
    if bool::nondet() {
        m1.collaterals.set(i, m1.collaterals.get(i).unwrap_or(0) + 10);

    }
    let m0: T = e.storage().persistent().get(&u).unwrap();
    certora::assert!(m1.collaterals.get(i).unwrap() >= m0.collaterals.get(i).unwrap());
}

#[derive(Clone,Nondet)]
#[contracttype]
pub struct Reserve {
    pub asset: Address,        // the underlying asset address
    pub index: u32,            // the reserve index in the pool
    pub l_factor: u32,         // the liability factor for the reserve
    pub c_factor: u32,         // the collateral factor for the reserve
    pub max_util: u32,         // the maximum utilization rate for the reserve
    pub last_time: u64,        // the last block the data was updated
    pub scalar: i128,          // scalar used for positions, b/d token supply, and credit
    pub d_rate: i128,          // the conversion rate from dToken to underlying (9 decimals)
    pub b_rate: i128,          // the conversion rate from bToken to underlying (9 decimals)
    pub ir_mod: i128,          // the interest rate curve modifier (9 decimals)
    pub b_supply: i128,        // the total supply of b tokens
    pub d_supply: i128,        // the total supply of d tokens
    pub backstop_credit: i128, // the total amount of underlying tokens owed to the backstop
}

#[rule]
pub fn position_identity(env: &Env, user1: Address, user2: Address, v: Vec<i32>, mut m: Map<u32, u32>) {
    let mut pool = pool::Pool::load(env);
    for i in v.iter() {
        certora::require!(false, "ASDF");
        let mut reserve: pool::Reserve = nondet::nondet();//pool.load_reserve(env, &user2, true);
    }
    let u1 = storage::get_user_positions(env, &user1);
    let u2 = storage::get_user_positions(env, &user1);
    certora::assert!(u1.collateral.get(0) == u2.collateral.get(0));
}

#[rule]
pub fn position(env: &Env, user1: Address, u: u32) {
    // let mut pool = pool::Pool::load(env);
    let mut u1 = storage::get_user_positions(env, &user1);
    // for i in v.iter() {
    //     // let mut reserve: pool::Reserve = nondet::nondet();//pool.load_reserve(env, &user2, true);
        u1.collateral.set(u, 10 + u1.collateral.get(u).unwrap_or(0));
    // }
    let u2 = storage::get_user_positions(env, &user1);
    certora::assert!(u1.collateral.get(u).unwrap() >= u2.collateral.get(u).unwrap());
}

#[rule]
pub fn cex1(env: &Env) {
    let m1: Map<u32, u32> = soroban_sdk::map![env];
    let m2 = soroban_sdk::map![env];
    certora::assert!(m1 == m2);
}
