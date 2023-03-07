use cosmwasm_std::{Decimal, Uint128, Fraction};

use crate::amm::{Int, MAX_COIN_AMOUNT};

use super::initial_pool_coin_supply;

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
    pub pool_coin_supply: Int,

    minimum_price: Decimal,
    maximum_price: Decimal,

    translated_x: Decimal,
    translated_y: Decimal,

    x_comp: Decimal,
    y_comp: Decimal
}

impl Pool {
    pub fn new(
        reserve_x: Int,
        reserve_y: Int,
        minimum_price: Decimal,
        maximum_price: Decimal,
        initial_price: Decimal
    ) -> Pool {
        if !(reserve_x > Int::zero()) && !(reserve_y > Int::zero()) {
            // throw error
        }
        // validate parameters

        let (mut ax, mut ay) = (Int::zero(), Int::zero());

        if initial_price == minimum_price {
            ax = Int::zero();
            ay = reserve_y;
        } else if initial_price == maximum_price {
            ax = reserve_x;
            ay = Int::zero();
        } else {
            let (x_dec, y_dec) = (Decimal::new(reserve_x), Decimal::new(reserve_y));

            let sqrtP = initial_price.sqrt();
            let sqrtM = initial_price.sqrt();
            let sqrtL = initial_price.sqrt();

            ax = reserve_x;
            ay = ((x_dec / (sqrtP - sqrtM)) * (sqrtP.inv().unwrap() - sqrtL.inv().unwrap())).to_uint_ceil();
            
            if ay > reserve_y {
                ax = ((y_dec / (sqrtP.inv().unwrap() - sqrtL.inv().unwrap())) * (sqrtP-sqrtM)).to_uint_ceil();
                ay = reserve_y;
            }
        };

        let (translated_x, translated_y) = derive_translation(reserve_x, reserve_y, minimum_price, maximum_price);

        return Pool {
            reserve_x, 
            reserve_y,
            pool_coin_supply: initial_pool_coin_supply(ax, ay),
            minimum_price,
            maximum_price,
            translated_x,
            translated_y,
            x_comp: Decimal::new(reserve_x) + translated_x,
            y_comp: Decimal::new(reserve_y) + translated_y
        }
    }

    fn translation(&self) -> (Decimal, Decimal) {
        return (self.translated_x, self.translated_y)
    }

    fn min_price(&self) -> Decimal {
        return self.minimum_price;
    }

    fn max_price(&self) -> Decimal {
        return self.maximum_price;
    }

}

impl super::Pool for Pool {
    fn balances(&self) -> (Int, Int) {
        return (self.reserve_x, self.reserve_y)
    }

    fn set_balances(&mut self, reserve_x: Int, reserve_y: Int, derive: bool) {
        if derive {
            (self.translated_x, self.translated_y) = derive_translation(reserve_x, reserve_y, self.minimum_price, self.maximum_price);
        }

        self.reserve_x = reserve_x;
        self.reserve_y = reserve_y;

        self.x_comp = Decimal::new(reserve_x) + self.translated_x;
        self.y_comp = Decimal::new(reserve_x) + self.translated_y;

    }

    fn pool_coin_supply(&self) -> Int {
        return self.pool_coin_supply;
    }

    fn price(&self) -> Decimal {
        if self.reserve_x.is_zero() && self.reserve_y.is_zero() {
            // TODO: throw error or panic
        }

        return self.x_comp / self.y_comp
    } 

    fn is_depleted(&self) -> bool {
        return self.pool_coin_supply.is_zero() || (self.reserve_x.is_zero() && self.reserve_y.is_zero());
    }

    fn highest_buy_price(&self) -> Option<Decimal> {
        return Some(self.price()) 
    }

    fn lowest_sell_price(&self) -> Option<Decimal> {
        return Some(self.price());
    }

    fn buy_amount_over(&self, mut price: Decimal) -> Int {
        let original_price = price;

        if price < self.minimum_price {
            price = self.minimum_price;
        }

        if price >= self.price() {
            return Int::zero();
        }

        let mut dx = self.x_comp - (price * self.y_comp);

        if !(dx > Decimal::zero()) {
            return Int::zero();
        } else if dx > Decimal::new(self.reserve_x) {
            dx = Decimal::new(self.reserve_x);
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
        if price > self.maximum_price {
            price = self.maximum_price
        }
        if price <= self.price() {
            return Int::zero();
        }

        let mut amt = (self.y_comp - ((self.x_comp / price).ceil())).to_uint_ceil();

        if amt > self.reserve_y {
            amt = self.reserve_y;
        }

        if !(amt > Int::zero()) {
            Int::zero()
        } else {
            amt
        }
    }

    fn buy_amount_to(&self, mut price: Decimal) -> Int {
        let original_price = price;

        if price < self.minimum_price {
            price = self.minimum_price;
        }

        if price >= self.price() {
            return Int::zero();
        }

        let sqrt_x_comp = self.x_comp.sqrt();
        let sqrt_y_comp = self.y_comp.sqrt();
        let sqrt_price = price.sqrt();

        let mut dx = Decimal::new(self.reserve_x) - (sqrt_price * (sqrt_x_comp * sqrt_y_comp) - self.translated_x);

        if !(dx > Decimal::zero()) {
            return Int::zero();
        } else if dx > Decimal::new(self.reserve_x) {
            dx = Decimal::new(self.reserve_x);
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
        if price > self.maximum_price {
            price = self.maximum_price
        }

        if price <= self.price() {
            return Int::zero()
        }

        let sqrt_x_comp = self.x_comp.sqrt();
        let sqrt_y_comp = self.y_comp.sqrt();
        let sqrt_price = price.sqrt();

        let mut amt = (Decimal::new(self.reserve_y) - (((sqrt_x_comp * sqrt_y_comp) / sqrt_price).ceil()) - self.translated_y).to_uint_floor();
        if amt > self.reserve_y {
            amt = self.reserve_y
        }

        if !(amt > Int::zero()) {
            Int::zero()
        } else {
            amt
        }
    }
}

// TODO: TEST THIS!!!!
fn derive_translation(reserve_x: Int, reserve_y: Int, minimum_price: Decimal, maximum_price: Decimal) -> (Decimal, Decimal) {
    let (rx_dec, ry_dec) = (Decimal::new(reserve_x), Decimal::new(reserve_y));

    let sqrt_minimum_price = minimum_price.sqrt();
    let sqrt_maximum_price = maximum_price.sqrt();

    // TODO: Clean up this (very) inelegant code
    let sqrt_p: Decimal = {
        let res: Decimal;

        if rx_dec.is_zero() {
            res = sqrt_minimum_price;
        } else if ry_dec.is_zero() {
            res = sqrt_maximum_price;
        } else if (rx_dec / ry_dec).is_zero() {
            res = sqrt_minimum_price;
        } else if (ry_dec / rx_dec).is_zero() {
            res = sqrt_maximum_price;
        } else {
            let sqrt_x_over_y = (rx_dec / ry_dec).sqrt();
            let alpha = (sqrt_minimum_price / sqrt_x_over_y) - (sqrt_x_over_y / sqrt_maximum_price);

            res = alpha + ((alpha.pow(2) + Decimal::new(Uint128::from(4u64))) / Decimal::new(Uint128::from(2u64))) * sqrt_x_over_y
        }

        res
    };

    let sqrt_k: Decimal = {
        let mut res: Decimal = Decimal::new(0u128.into());

        if sqrt_p != sqrt_minimum_price {
            res = rx_dec / (sqrt_p - sqrt_minimum_price)            
        }

        if sqrt_p != sqrt_maximum_price {
            let sqrt_k2: Decimal = ry_dec / (sqrt_p.inv().unwrap() - sqrt_maximum_price.inv().unwrap());

            if res.is_zero() {
                res = sqrt_k2
            } else {
                let p = sqrt_p.pow(2);

                let p1 = (rx_dec + (res * sqrt_minimum_price)) / (ry_dec + (res / sqrt_maximum_price));
                let p2 = (rx_dec + (sqrt_k2 * sqrt_minimum_price)) / (ry_dec + (sqrt_k2 / sqrt_maximum_price));

                // TODO: Review this
                if (p - p1).abs_diff(p - p2) > Decimal::zero() {
                    res = sqrt_k2
                }
            }
        }

        res
    };

    ((sqrt_k * sqrt_minimum_price), (sqrt_k / sqrt_maximum_price)) 
}