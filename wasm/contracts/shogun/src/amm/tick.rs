use cosmwasm_std::Decimal;

use super::Int;


type TickPrecision = u64;

fn price_to_down_tick(price: Decimal, precision: TickPrecision) -> Decimal {
    todo!()
}

fn price_to_up_tick(price: Decimal, precision: TickPrecision) -> Decimal {
    todo!()
}

fn up_tick(price: Decimal, precision: TickPrecision) -> Decimal {
    todo!()
}

fn down_tick(price: Decimal, precision: TickPrecision) -> Decimal {
    todo!()
}

fn highest_tick(price: Decimal, precision: TickPrecision) -> Decimal {
    todo!()
}

fn lowest_tick(price: Decimal, precision: TickPrecision) -> Decimal {
    todo!()
}

fn tick_to_index(price: Decimal, precision: TickPrecision) -> Decimal {
    todo!()
}

fn tick_from_index(price: Decimal, precision: TickPrecision) -> Decimal {
    todo!()
}

fn round_tick_index(i: TickPrecision) -> TickPrecision {
    todo!()
}

fn round_price(price: Decimal, precision: TickPrecision) -> Decimal {
    todo!()

}

fn tick_gap(price: Decimal, precision: TickPrecision) -> Decimal {
    todo!()
}

// RNG in CosmWasm needs to be researched, so lets avoid implementing this method for now.
// fn random_tick()