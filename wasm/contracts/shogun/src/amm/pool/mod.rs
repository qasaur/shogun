use cosmwasm_std::Decimal;

use super::Int;

pub mod basic;
pub mod ranged;

trait Pool {
    fn balances(&self) -> (Int, Int);
    fn set_balances(&mut self, reserve_x: Int, reserve_y: Int, derive: bool);

    fn pool_coin_supply(&self) -> Int;
    fn price(&self) -> Decimal;
    fn is_depleted(&self) -> bool;

    fn highest_buy_price(&self) -> Option<Decimal>;
    fn lowest_sell_price(&self) -> Option<Decimal>;

    fn buy_amount_over(&self, price: Decimal) -> Int;
    fn sell_amount_under(&self, price: Decimal) -> Int;
    fn buy_amount_to(&self, price: Decimal) -> Int;
    fn sell_amount_to(&self, price: Decimal) -> Int;
}

/// InitialPoolCoinSupply returns ideal initial pool coin minting amount.
fn initial_pool_coin_supply(x: Int, y: Int) -> Int {
    // TODO: REALLY FIX THIS

    return Int::zero()
}