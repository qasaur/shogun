use std::{cmp::Ordering, collections::HashMap};

use cosmwasm_std::{Decimal, Uint64};

use super::Int;

#[derive(Debug, Clone, Eq, PartialEq)]

pub enum Direction {
    Buy,
    Sell
}

// TODO: implement to string
#[derive(Clone, Debug, Eq)]

pub struct Order {
    pub direction: Direction,
    pub price: Decimal, 
    pub amount: Int,

    pub offer_coin_amount: Int,
    pub open_amount: Int,
 
    pub paid_offer_coin_amount: Int,
    pub received_demand_coin_amount: Int
}


impl Order {
    pub fn new(dir: Direction, price: Decimal, amount: Int) -> Order {
        return Order {
            direction: dir.clone(),
            price,
            amount,
            offer_coin_amount: offer_coin_amount(dir, price, amount), 
            open_amount: amount,
            paid_offer_coin_amount: Int::zero(),
            received_demand_coin_amount: Int::zero(),
        }
    }

    fn is_matched(&self) -> bool {
        return self.open_amount < self.amount
    }

    pub fn get_batch_id(&self) -> u64 {
        return 0;
    }
}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> Ordering {
        self.amount.cmp(&other.amount)
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount
    }
}

pub fn total_amount(orders: Vec<Order>) -> Int {
    orders
        .iter()
        .map(|order| order.amount)
        .sum()
}

pub fn total_matchable_amount(orders: Vec<Order>, price: Decimal) -> Int {
    orders
        .iter()
        .map(|order| matchable_amount(order, price))
        .sum()
}

// TODO: To string
pub enum PriceDirection {
    Staying,
    Increasing,
    Decreasing
}

pub fn fill_order(order: Order, amount: Int, price: Decimal) -> Int {
    let matchable_amount = matchable_amount(&order, price);
    if amount > matchable_amount {
        // panic
        todo!();
    }
    
    let quote_coin_diff: Int;
    let paid: Int;
    let received: Int;

    match order.direction {
        Direction::Buy => {
            paid = amount.mul_ceil(price);
            received = amount;
            quote_coin_diff = paid;
        },
        Direction::Sell => {
            paid = price * amount;
            received = amount.mul_ceil(price);
            quote_coin_diff = received; // review this
        }
    };

    order.paid_offer_coin_amount = order.paid_offer_coin_amount + paid;
    order.received_demand_coin_amount = order.received_demand_coin_amount + received;
    order.open_amount = order.open_amount - amount;

    return quote_coin_diff
}

pub fn fulfill_order(order: Order, price: Decimal) -> Int {
    let quote_coin_diff = Int::zero();
    let matchable_amount = matchable_amount(&order, price);
    if matchable_amount > Int::zero() {
        quote_coin_diff = quote_coin_diff + fill_order(order, matchable_amount, price);
    }

    quote_coin_diff
}

pub fn fulfill_orders(orders: Vec<Order>, price: Decimal) -> Int {
    let quote_coin_diff = Int::zero();

    for order in orders.iter() {
        quote_coin_diff = quote_coin_diff + fulfill_order(order.to_owned(), price)
    }

    quote_coin_diff
}

pub fn distribute_order_amount_to_orders(orders: Vec<Order>, amount: Int, price: Decimal) -> Int {
    let total_amount = total_amount(orders);
    let total_matched_amount = Int::zero();
    let mut matched_amount_by_order: HashMap<Order, Int> = HashMap::new();

    for order in orders.iter() {
        let matchable_amt = matchable_amount(order, price);
        if matchable_amt.is_zero() {
            continue
        }

        let order_amount: Decimal = Decimal::new(order.amount);
        let proportion = order_amount / total_amount;
        matchable_amt = Int::min(matchable_amt, proportion * amount);
        if matchable_amt > Int::zero() {
            todo!()
        }
    };

    return Int::zero()
}

/// MatchableAmount returns the matchable amount of an order based on
/// its remaining offer coin and price.
pub fn matchable_amount(order: &Order, price: Decimal) -> Int {
    let matchable_amount = match order.direction {
        Direction::Buy => {
            let remaining_offer_coin_amount = order.offer_coin_amount - order.paid_offer_coin_amount;

            let potential_amount: Decimal = Decimal::new(remaining_offer_coin_amount) / price;

            return order.open_amount.min(potential_amount.to_uint_floor())
        },
        Direction::Sell => order.open_amount,
    };

    if {price * matchable_amount}.is_zero() {
        Int::zero()
    } else {
        matchable_amount
    }
}

/// Returns the minimum offer coin amount for a given order direction, price, and
pub fn offer_coin_amount(dir: Direction, price: Decimal, amount: Int) -> Int {
    match dir {
        Direction::Buy => {
            let product: Decimal = price * Decimal::new(amount);

            product.to_uint_ceil()
        },
        Direction::Sell => amount,
    }
}


#[cfg(test)]
mod tests {
    use rstest::rstest;

}