use cosmwasm_std::{Decimal, Uint64};

// TODO: To string
pub enum PriceDirection {
    Staying,
    Increasing,
    Decreasing
}

pub fn fill_order(order: order::Order, amount: cosmwasm_std::Uint64, price: Decimal) -> Uint64 {
    let quoteCoinDiff: Uint64;

    let matchableAmount = matchable_amount(order, price);

    if amount > matchableAmount {
        // TODO: Throw error
        return
    }

    let paid: Uint64;
    let received: Uint64;

    match order.get_direction() {
        order::Direction::Buy => {
            paid = price * amount;
            received = amount;
            quoteCoinDiff = paid;
        },
        order::Direction::Sell => {
            paid = price * amount;
            received 
        }
    }
}