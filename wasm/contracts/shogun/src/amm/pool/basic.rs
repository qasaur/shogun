use cosmwasm_std::Decimal;
use lazy_static::lazy_static;

use crate::amm::{Int, MAX_COIN_AMOUNT};

use super::{initial_pool_coin_supply};

lazy_static! {
    static ref MIN_POOL_PRICE: Decimal = Decimal::from_atomics(Int::new(1), 15).unwrap();
    static ref MAX_POOL_PRICE: Decimal = Decimal::from_atomics(Int::new(1), 20).unwrap();
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("zero reserve amount")]
    ZeroReserve,
    #[error("pool price is lower than minimum price {0}")]
    PoolPriceTooLow(Decimal),
    #[error("pool price is higher than maximum price {0}")]
    PoolPriceTooHigh(Decimal),
}

pub struct Pool {
    reserve_x: Int,
    reserve_y: Int,

    pub pool_coin_supply: Int
}

impl Pool {
    pub fn new(reserve_x: Int, reserve_y: Int) -> Result<Pool, Error> {
        if reserve_x.is_zero() || reserve_y.is_zero() {
            return Err(Error::ZeroReserve);
        }

        let price = Decimal::new(reserve_x) / Decimal::new(reserve_y);

        if price < *MIN_POOL_PRICE {
            return Err(Error::PoolPriceTooLow(*MIN_POOL_PRICE));
        } else if price > *MAX_POOL_PRICE {
            return Err(Error::PoolPriceTooHigh(*MAX_POOL_PRICE));
        }

        return Ok(Pool {
            reserve_x,
            reserve_y,
            pool_coin_supply: initial_pool_coin_supply(reserve_x, reserve_y)
        })
    }
}

impl super::Pool for Pool {
    fn balances(&self) -> (Int, Int) {
        return (self.reserve_x, self.reserve_y)
    }

    fn set_balances(&mut self, reserve_x: Int, reserve_y: Int, _: bool) {
        self.reserve_x = reserve_x;
        self.reserve_y = reserve_y;
    }

    fn pool_coin_supply(&self) -> Int {
        return self.pool_coin_supply
    }

    fn price(&self) -> Decimal {
        if self.reserve_x.is_zero() || self.reserve_y.is_zero() {
            todo!()
        }

        return Decimal::new(self.reserve_x) / Decimal::new(self.reserve_y)
    }

    fn is_depleted(&self) -> bool {
        return self.pool_coin_supply.is_zero() || self.reserve_x.is_zero() || self.reserve_y.is_zero()
    }

    fn highest_buy_price(&self) -> Option<Decimal> {
        return Some(self.price())
    }

    fn lowest_sell_price(&self) -> Option<Decimal> {
        return Some(self.price())
    }

    fn buy_amount_over(&self, mut price: Decimal) -> Int {
        let original_price = self.price();

        if price < *MIN_POOL_PRICE {
            price = *MIN_POOL_PRICE;
        }

        if price >= self.price() {
            return Int::zero()
        }

        let dx = Decimal::new(self.reserve_x - (price * self.reserve_y));
        if !(dx > Decimal::zero()) {
            return Int::zero()
        }

        dx.checked_div(original_price)
            .map_or_else(|_| *MAX_COIN_AMOUNT, |mut amt| {
                if amt > *MAX_COIN_AMOUNT {
                    amt = *MAX_COIN_AMOUNT
                }
                amt
            })
            .to_uint_floor() // Truncate always rounds down in Go Cosmos-SDK
    }

    fn sell_amount_under(&self, mut price: Decimal) -> Int {
        if price > *MAX_POOL_PRICE {
            price = *MAX_POOL_PRICE;
        }

        if price <= self.price() {
            return Int::zero()
        }

        let amt = (Decimal::new(self.reserve_y) - (Decimal::new(self.reserve_x) / price)).to_uint_ceil();
        if !(amt > Int::zero()) {
            Int::zero()
        } else {
            amt
        }
    }

    fn buy_amount_to(&self, mut price: Decimal) -> Int {
        let original_price = price;

        if price < *MIN_POOL_PRICE {
            price = *MIN_POOL_PRICE
        }

        if price >= self.price() {
            return Int::zero()
        }

        let sqrt_rx = Decimal::new(self.reserve_x).sqrt();
        let sqrt_ry = Decimal::new(self.reserve_y).sqrt();
        let sqrt_price = price.sqrt();

        let dx = Decimal::new(self.reserve_x) - (sqrt_price * sqrt_rx * sqrt_ry); // NOTE: Something might go wrong here
        
        if !(dx > Decimal::zero()) {
            return Int::zero()
        }

        dx.checked_div(original_price)
            .map_or_else(|_| *MAX_COIN_AMOUNT, |mut amt| {
                if amt > *MAX_COIN_AMOUNT {
                    amt = *MAX_COIN_AMOUNT
                }
                amt
            })
            .to_uint_floor() // Truncate always rounds down in Go Cosmos-SDK
    }

    fn sell_amount_to(&self, mut price: Decimal) -> Int {
        if price > *MAX_POOL_PRICE {
            price = *MAX_POOL_PRICE
        }
        
        if price <= self.price() {
            return Int::zero()
        }

        let sqrt_rx = Decimal::new(self.reserve_x).sqrt();
        let sqrt_ry = Decimal::new(self.reserve_y).sqrt();
        let sqrt_price = price.sqrt();

        let amt = (Decimal::new(self.reserve_y) - (sqrt_rx * sqrt_ry / sqrt_price)).to_uint_floor();
        if !(amt > Int::zero()) {
            return Int::zero()
        } else {
            amt
        }
    }

}