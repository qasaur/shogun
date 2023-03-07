use std::ops::Add;

use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo};

use crate::types::liquidity::{Pair, Pool, DepositRequest, WithdrawRequest, Order};


pub fn get_last_pair_id(ctx: (DepsMut, Env, MessageInfo)) -> u64 {
    todo!()
}

pub fn set_last_pair_id(ctx: (DepsMut, Env, MessageInfo), id: u64) {
    todo!()
}

pub fn get_pair(ctx: (DepsMut, Env, MessageInfo), id: u64) -> Option<Pair> {
    todo!()
}

pub fn get_pair_by_denoms(ctx: (DepsMut, Env, MessageInfo), base_coin_denomination: String, quote_coin_denomination: String) -> Option<Pair> {
    todo!()
}

pub fn set_pair(ctx: (DepsMut, Env, MessageInfo), pair: Pair) {
    todo!()
}

pub fn set_pair_index(ctx: (DepsMut, Env, MessageInfo), base_coin_denomination: String, quote_coin_denomination: String, pair_id: u64) {
    todo!()
}

pub fn set_pair_lookup_index(ctx: (DepsMut, Env, MessageInfo), denom_a: String, denom_b: String, pair_id: u64) {
    todo!()   
}

// pub fn iterate_all_pairs(ctx: (DepsMut, Env, MessageInfo), )

pub fn get_all_pairs(ctx: (DepsMut, Env, MessageInfo)) -> Vec<Pair> {
    todo!()
}

pub fn get_last_pool_id(ctx: (DepsMut, Env, MessageInfo)) -> u64 {
    todo!()
}

pub fn set_last_pool_id(ctx: (DepsMut, Env, MessageInfo), id: u64) {
    todo!()
}

pub fn get_pool(ctx: (DepsMut, Env, MessageInfo), id: u64) -> Option<Pool> {
    todo!()
}

pub fn get_pool_by_reserve_address(ctx: (DepsMut, Env, MessageInfo), reserve_addr: Addr) -> Option<Pool> {
    todo!()
}

pub fn set_pool(ctx: (DepsMut, Env, MessageInfo), pool: Pool) {
    todo!()
}

pub fn set_pool_by_reserve_index(ctx: (DepsMut, Env, MessageInfo), pool: Pool) {
    todo!()   
}

pub fn set_pool_by_pair_index(ctx: (DepsMut, Env, MessageInfo), pool: Pool) {
    todo!()   
}

//pub fn iterateallpools
//pub fn iterate pools by pair

pub fn get_all_pools(ctx: (DepsMut, Env, MessageInfo)) -> Vec<Pool> {
    todo!()
}

pub fn get_pools_by_pair(ctx: (DepsMut, Env, MessageInfo), pair_id: u64) -> Vec<Pool> {
    todo!()
}

pub fn get_deposit_request(ctx: (DepsMut, Env, MessageInfo), pool_id: u64, id: u64) -> Option<DepositRequest> {
    todo!()
}

pub fn set_deposit_request(ctx: (DepsMut, Env, MessageInfo), req: DepositRequest) {
    todo!()
}

// pub fn iterate all deposit requests
// pub fn iterate deposit requests by depositor

pub fn get_all_deposit_requests(ctx: (DepsMut, Env, MessageInfo)) -> Vec<DepositRequest> {
    todo!()
}

pub fn get_all_deposit_requests_by_depositor(ctx: (DepsMut, Env, MessageInfo), depositor: Addr) -> Vec<DepositRequest> {
    todo!()
}

pub fn delete_deposit_request(ctx: (DepsMut, Env, MessageInfo), req: DepositRequest) {
    todo!()
}

pub fn delete_deposit_request_index(ctx: (DepsMut, Env, MessageInfo), req: DepositRequest) {
    todo!()
}

pub fn get_withdraw_request(ctx: (DepsMut, Env, MessageInfo), pool_id: u64, id: u64) -> Option<DepositRequest> {
    todo!()
}

pub fn set_withdraw_request(ctx: (DepsMut, Env, MessageInfo), req: WithdrawRequest) {
    todo!()
}

// pub fn iterate all withdrawal requests
// pub fn iterate withdrawal requests by withdrawer

pub fn get_all_withdraw_requests(ctx: (DepsMut, Env, MessageInfo)) -> Vec<WithdrawRequest> {
    todo!()
}

pub fn get_all_withdraw_requests_by_withdrawer(ctx: (DepsMut, Env, MessageInfo), withdrawer: Addr) -> Vec<WithdrawRequest> {
    todo!()
}

pub fn delete_withdraw_request(ctx: (DepsMut, Env, MessageInfo), req: WithdrawRequest) {
    todo!()
}

pub fn delete_withdraw_request_index(ctx: (DepsMut, Env, MessageInfo), req: WithdrawRequest) {
    todo!()
}

pub fn get_order(ctx: (DepsMut, Env, MessageInfo), pair_id: u64, id: u64) -> Option<Order> {
    todo!()
}

pub fn set_order(ctx: (DepsMut, Env, MessageInfo), pair_id: u64, id: u64) {
    todo!()
}

pub fn set_order_index(ctx: (DepsMut, Env, MessageInfo), order: Order) {
    todo!()
}

// iterate all orders
// iterate orders by pair
// iterate orders by orderer

pub fn get_all_orders(ctx: (DepsMut, Env, MessageInfo)) -> Vec<Order> {
    todo!()
}

pub fn get_orders_by_pair(ctx: (DepsMut, Env, MessageInfo), pair_id: u64) -> Vec<Order> {
    todo!()
}

pub fn get_orderers_by_orderer(ctx: (DepsMut, Env, MessageInfo), orderer: Addr) -> Vec<Order> {
    todo!()
}

pub fn delete_order(ctx: (DepsMut, Env, MessageInfo), order: Order) {
    todo!()
}

pub fn delete_order_index(ctx: (DepsMut, Env, MessageInfo), order: Order) {
    todo!()
}