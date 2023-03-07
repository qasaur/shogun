use cosmwasm_std::{Uint128, Uint64, Decimal};
use lazy_static::lazy_static;

// pub mod matcher;
pub mod order;
pub mod orderbook;
pub mod pool;
pub mod tick;
pub mod view;

lazy_static! {
    static ref MIN_COIN_AMOUNT: Int = Int::new(100);
    static ref MAX_COIN_AMOUNT: Decimal = Decimal::from_atomics(1u128, 4u32).unwrap();
}

type Int = Uint128;