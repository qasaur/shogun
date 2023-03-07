use cosmwasm_std::Decimal;

use super::Int;


struct OrderView {
    buy_amt_acc_sums: Vec<AmountAccountSum>,
    sell_amt_acc_sums: Vec<AmountAccountSum>
}

impl OrderView {
    fn _match(&self) {
        if self.buy_amt_acc_sums.len() == 0 || self.sell_amt_acc_sums.len() == 0 {
            return
        }

        todo!()
    }

    fn highest_buy_price(&self) -> Option<Decimal> {
        todo!()
    }

    fn lowest_sell_price(&self) -> Option<Decimal> {
        todo!()
    }

    fn buy_amount_over(&self, price: Decimal, inclusive: bool) -> Int {
        todo!()
    }

    fn buy_amount_under(&self, price: Decimal, inclusive: bool) -> Int {
        todo!()
    }

    fn sell_amount_over(&self, price: Decimal, inclusive: bool) -> Int {
        todo!()
    }

    fn sell_amount_under(&self, price: Decimal, inclusive: bool) -> Int {
        todo!()
    }
}

struct AmountAccountSum {
    price: Decimal,
    sum: Int
}