use std::str::FromStr;

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, CosmosMsg, StdResult, BankMsg, Uint128, Coin, Decimal};

use crate::{types::{tx::{MsgCreatePool, MsgCreateRangedPool}, liquidity::Pool, }, store::{get_pair, get_last_pool_id, set_last_pool_id, set_pool, set_pool_by_reserve_index, set_pool_by_pair_index}, contract::ContractError, };


pub fn create_pool(ctx: (DepsMut, Env, MessageInfo), msg: MsgCreatePool) -> Result<Response, ContractError> {
    // validate msg here

    let pair = get_pair(ctx, msg.pair_id)
        .ok_or(|err| Err(err))?; // TODO: More descriptive errors

    let (x, y) = (msg.deposit_coins.get)

    let ammPool = crate::amm::pool::basic::Pool::new(x, y).unwrap();

    let pool_id = get_last_pool_id(ctx) + 1;
    set_last_pool_id(ctx, pool_id);

    let pool = Pool::new();
    pool.id = pool_id;
    pool.pair_id = pair.id;
    pool.creator = msg.creator;

    set_pool(ctx, pool);
    set_pool_by_reserve_index(ctx, pool);
    set_pool_by_pair_index(ctx, pool);

    let msgs: Vec<BankMsg> = vec![];

    msgs.push(BankMsg::Send {
        to_address: pool.reserve_address,
        amount: vec![
            cosmwasm_std::Coin::new(msg.deposit_coins.first().unwrap().amount.parse::<u128>().unwrap(), msg.deposit_coins.first().unwrap().denom),
            cosmwasm_std::Coin::new(msg.deposit_coins.last().unwrap().amount.parse::<u128>().unwrap(), msg.deposit_coins.last().unwrap().denom)
            ],
    });

    // collect native fee

    let ps = Uint128::min(ammPool.pool_coin_supply, 100.into()); // TODO: Fix min initial coin supply

    let pool_coin = Coin::new(
        ps.into(),
        pool.pool_coin_denom
    );

    let resp = Response::new()
        .add_messages(msgs);

    Ok(resp)
}

pub fn create_ranged_pool(ctx: (DepsMut, Env, MessageInfo) msg: MsgCreateRangedPool) -> Result<Response, ()> {
    let pair = get_pair(ctx, msg.pair_id)
        .ok_or(|err| Err(err))?; // TODO: More descriptive errors

    let (x, y) = (msg.deposit_coins.get)

    let ammPool = crate::amm::pool::ranged::Pool::new(x, y, Decimal::from_str(msg.min_price.as_str()).unwrap(), Decimal::from_str(msg.max_price.as_str()).unwrap(), Decimal::from_str(msg.initial_price.as_str()).unwrap());

    let pool_id = get_last_pool_id(ctx) + 1;
    set_last_pool_id(ctx, pool_id);

    let pool = Pool::new();
    pool.id = pool_id;
    pool.pair_id = pair.id;
    pool.creator = msg.creator;

    set_pool(ctx, pool);
    set_pool_by_reserve_index(ctx, pool);
    set_pool_by_pair_index(ctx, pool);

    let msgs: Vec<BankMsg> = vec![];

    msgs.push(BankMsg::Send {
        to_address: pool.reserve_address,
        amount: vec![
            cosmwasm_std::Coin::new(msg.deposit_coins.first().unwrap().amount.parse::<u128>().unwrap(), msg.deposit_coins.first().unwrap().denom),
            cosmwasm_std::Coin::new(msg.deposit_coins.last().unwrap().amount.parse::<u128>().unwrap(), msg.deposit_coins.last().unwrap().denom)
            ],
    });

    // collect native fee

    let ps = Uint128::min(ammPool.pool_coin_supply, 100.into()); // TODO: Fix min initial coin supply

    let pool_coin = Coin::new(
        ps.into(),
        pool.pool_coin_denom
    );

    let resp = Response::new()
        .add_messages(msgs);

    Ok(resp)
}

pub fn deposit(ctx: (DepsMut, Env, MessageInfo)) {

}

pub fn withdraw(ctx: (DepsMut, Env, MessageInfo)) {

}

fn execute_deposit_request(ctx: (DepsMut, Env, MessageInfo)) {

}

fn finish_deposit_request(ctx: (DepsMut, Env, MessageInfo)) {

}

fn execute_withdraw_request(ctx: (DepsMut, Env, MessageInfo)) {

}

fn finish_withdraw_request(ctx: (DepsMut, Env, MessageInfo)) {

}