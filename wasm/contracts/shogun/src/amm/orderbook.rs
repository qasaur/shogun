use std::{cmp::Ordering, collections::{self, HashMap}};

use cosmwasm_std::Decimal;
use itertools::Itertools;

use super::{order::{Order, matchable_amount, Direction, total_matchable_amount, distribute_order_amount_to_orders, fulfill_orders}, Int};


struct Orderbook {
    buys: Ticks,
    sells: Ticks
}

impl Orderbook {
    fn new(orders: Vec<Order>) -> Orderbook {
        let mut ob = Orderbook {
            buys: Ticks {
                ticks: vec![], 
                price_increasing: false,
            },
            sells: Ticks {
                ticks: vec![], 
                price_increasing: true,
            }
        };

        ob.add_order(orders);
        return ob
    }

    fn add_order(&mut self, orders: Vec<Order>) {
        orders.iter()
            .filter(|order| (matchable_amount(order, order.price) > Int::zero()))
            .for_each(|order| {
                match order.direction {
                    super::order::Direction::Buy => self.buys.add_order(order.clone()),
                    super::order::Direction::Sell => self.sells.add_order(order.clone()) 
                }
            });
    }

    fn orders(&self) -> Vec<Order> {
        let mut orders: Vec<Order> = vec![];

        for tick in self.buys.ticks {
            orders.extend(tick.orders);
        }

        for tick in self.sells.ticks {
            orders.extend(tick.orders);
        }

        orders
    }
    fn buy_orders_at(&self, price: Decimal) -> Option<Vec<Order>> {
        return self.buys.orders_at(price)
    }

    fn sell_orders_at(&self, price: Decimal) -> Option<Vec<Order>> {
        return self.sells.orders_at(price)
    }

    fn highest_price(&self) -> Option<Decimal> {
        let highestBuyPrice = self.buys.highest_price();
        let highestSellPrice = self.sells.highest_price();

        if highestBuyPrice.is_some() && highestSellPrice.is_some() {
            return Some(Decimal::max(highestBuyPrice.unwrap().0, highestSellPrice.unwrap().0))
        } else if highestBuyPrice.is_some() {
            return Some(highestBuyPrice.unwrap().0);
        } else if highestSellPrice.is_some() {
            return Some(highestSellPrice.unwrap().0);
        } else {
            return None
        }
    }

    fn lowest_price(&self) -> Option<Decimal> {
        let lowestBuyPrice = self.buys.lowest_price();
        let lowestSellPrice = self.sells.lowest_price();

        if lowestBuyPrice.is_some() && lowestSellPrice.is_some() {
            return Some(Decimal::max(lowestBuyPrice.unwrap().0, lowestSellPrice.unwrap().0))
        } else if lowestBuyPrice.is_some() {
            return Some(lowestBuyPrice.unwrap().0);
        } else if lowestSellPrice.is_some() {
            return Some(lowestSellPrice.unwrap().0);
        } else {
            return None
        }
    }

    fn find_matchable_amount_at_single_price(match_price: Decimal) -> Option<Int> {
        struct Side {
            ticks: Vec<Tick>,
            total_matchable_amount: Int, 
            i: Int,
            partial_match_amount: Int
        }

        todo!()
    }

    fn match_at_single_price(match_price: Decimal) -> Option<Int> {
        todo!()
    }

    fn price_direction(last_price: Decimal) -> Direction {
        todo!()
    }

    fn matchx(last_price: Decimal) -> Option<(Decimal, Int)> {
        todo!()
    }

    // TODO: Write string representations
}

struct Ticks {
    ticks: Vec<Tick>,
    price_increasing: bool
}

enum TickPrice {
    Index(usize),
    Exact(usize)
}

impl Ticks {
    fn find_price(&self, price: Decimal) -> TickPrice {
        let seek = self.ticks.binary_search_by(|probe| {
            if self.price_increasing {
                if probe.price == price {
                    return Ordering::Equal
                } else if probe.price > price {
                    return Ordering::Greater
                } else {
                    return Ordering::Less
                }
            } else {
                if probe.price == price {
                    return Ordering::Equal
                } else if probe.price > price {
                    return Ordering::Less
                } else {
                    return Ordering::Greater
                }
            }
        });

        seek.map_or_else(|x| TickPrice::Index(x), |x| {
            if self.ticks.get(x).unwrap().price == price {
                TickPrice::Exact(x)
            } else {
                TickPrice::Index(x)
            }
        })
    }

    fn add_order(&mut self, order: Order) {
        match Ticks::find_price(self, order.price) {
            TickPrice::Index(i) => {
                if i < self.ticks.len() {
                    self.ticks.insert(i, Tick::new(order)) // NOTE: This might go wrong..
                } else {
                    self.ticks.push(Tick::new(order))
                }
            },
            TickPrice::Exact(i) => self.ticks.get_mut(i).unwrap().add_order(order),
        }
    }

    fn orders_at(&self, price: Decimal) -> Option<Vec<Order>> {
        match Ticks::find_price(self, price) {
            TickPrice::Index(i) => Some(self.ticks.get(i).unwrap().orders),
            TickPrice::Exact(i) => None,
        }
    }

    fn highest_price(&self) -> Option<(Decimal, u64)>  {
        if self.ticks.len() == 0 {
            return None
        }

        if self.price_increasing {
            return Some((self.ticks.get(self.ticks.len() -1).unwrap().price, (self.ticks.len() - 1).try_into().unwrap()))
        } else {
            return Some((self.ticks.first().unwrap().price, 0))
        }
    }

    fn lowest_price(&self) -> Option<(Decimal, u64)> {
        if self.ticks.len() == 0 {
            return None
        }

        if self.price_increasing {
            return Some((self.ticks.first().unwrap().price, 0))
        } else {
            return Some((self.ticks.get(self.ticks.len() -1).unwrap().price, (self.ticks.len() - 1).try_into().unwrap()))
        }
    }
}

struct Tick {
    price: Decimal,
    orders: Vec<Order>
}

impl Tick {
    fn new(order: Order) -> Tick {
        return Tick {
            price: order.price, 
            orders: vec![order],
        }
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order)
    }

    fn distribute_order_amount(&self, amount: Int, price: Decimal) -> Int {
        let remaining_amount = amount;
        let quote_coin_difference = Int::zero();
        let groups = group_orders_by_batch_id(self.orders);

        for group in groups.iter() {
            let openAmt = total_matchable_amount(group.orders, price);
            if openAmt.is_zero() {
                continue
            }

            if remaining_amount >= openAmt {
                quote_coin_difference = quote_coin_difference + fulfill_orders(group.orders, price);
                remaining_amount = remaining_amount - openAmt;
            } else {
                group.orders.sort();
                quote_coin_difference = quote_coin_difference + distribute_order_amount_to_orders(group.orders, remaining_amount, price);
                remaining_amount = Int::zero();
            }
            
            if remaining_amount.is_zero() {
                break
            }
        }

        return quote_coin_difference
    } 
}

struct OrderGroup {
    batch_id: u64,
    orders: Vec<Order>
}

fn group_orders_by_batch_id(orders: Vec<Order>) -> Vec<OrderGroup> {
    let group_by_batch_id:HashMap<u64, OrderGroup> = HashMap::new();

    for order in orders.iter() {
        match group_by_batch_id.get(&order.get_batch_id()) {
            Some(og) => og.orders.push(*order),
            None => {
                todo!();
                // let group = OrderGroup {
                //     batch_id: order.get_batch_id();
                //     orders: 
                // }
            },
        }
    }

    group_by_batch_id.iter()
        .map(|g| *g.1)
        .collect_vec()
}