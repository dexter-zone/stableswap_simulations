use cosmwasm_std::{Decimal256, StdError, StdResult, Uint128, Uint256, Uint64};
mod lib;

fn main() {
    println!("Hello, world!");

    let amp = Uint64::from(100u64);
    let pool1 = Uint128::from(1242_000000u128);
    let pool2 = Uint128::from(1542_000000u128);
    let pool3 = Uint128::from(1456_000000u128);

    // let model = lib::StableSwapModel::new(
    //     amp.u64().into(),
    //     vec![pool1.u128(), pool2.u128(), pool3.u128()],
    //     3,
    // );

    // Dprev: 0
    // xp: [1242000000, 1542000000, 1456000000]
    // S: 4240000000
    // D: 4240000000
    // self.A: 100, self.n:3
    // Ann: 300
    // Start WHILE Loop: while abs(D - Dprev) > 1:
    // Start FOR Loop: D_P = D_P * D / (_x * N_COINS):
    // D_P: 4240000000
    // denominator: 3726000000
    // new D_P: 4824906065
    // D_P: 4824906065
    // denominator: 4626000000
    // new D_P: 4422309060
    // D_P: 4422309060
    // denominator: 4368000000
    // new D_P: 4292717585
    // new D_P (calculated via D_P = D_P * D / (_x * N_COINS) ):: 4292717585
    // Dprev: 4240000000
    // D: 4239826043 = (Ann * S + D_P * self.n) * D // ((Ann - 1) * D + (self.n + 1) * D_P)
    // where, Ann * S:1272000000000 | D_P * self.n:12878152755 | (Ann - 1) * D:1267707986857 | (self.n + 1) * D_P:17170870340
    // counter: 1
    // Start FOR Loop: D_P = D_P * D / (_x * N_COINS):
    // D_P: 4239826043
    // denominator: 3726000000
    // new D_P: 4824510165
    // D_P: 4824510165
    // denominator: 4626000000
    // new D_P: 4421764773
    // D_P: 4421764773
    // denominator: 4368000000
    // new D_P: 4292013150
    // new D_P (calculated via D_P = D_P * D / (_x * N_COINS) ):: 4292013150
    // Dprev: 4239826043
    // D: 4239826042 = (Ann * S + D_P * self.n) * D // ((Ann - 1) * D + (self.n + 1) * D_P)
    // where, Ann * S:1272000000000 | D_P * self.n:12876039450 | (Ann - 1) * D:1267707986558 | (self.n + 1) * D_P:17168052600
    // counter: 2
    // sim_d: 4239826042
    // let sim_d = model.sim_d();
    // println!("sim_d: {}", sim_d);

    // ---------------xp-------------------------------xp-------------------------------xp----------------

    pub const NATIVE_TOKEN_PRECISION: u8 = 6;
    let amp = 100u64;
    let pool1 = Uint128::from(1546325_000000u128);
    let pool2 = Uint128::from(1728525_000000u128);
    let pool3 = Uint128::from(1335325_000000u128);
    let offer_amount = Uint128::from(673_000000u128);

    let model = lib::StableSwapModel::new(
        amp as u128,
        vec![pool1.u128(), pool2.u128(), pool3.u128()],
        3,
    );
    // pool1 --> pool2
    let sim_y = model.sim_y(0, 1, pool1.u128() + offer_amount.u128());
    println!("sim_y: {}", sim_y);
}
