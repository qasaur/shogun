#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128,
};
use cw20::{Cw20Coin, Cw20CoinVerified, Cw20ExecuteMsg, Cw20ReceiveMsg};
use sylvia::contract;

use crate::{pair::create_pair, pool::{create_pool, create_ranged_pool, deposit, withdraw}, swap::{limit_order, market_order, cancel_order, cancel_all_orders}, types::{tx::{MsgCreatePair, MsgCreatePool, MsgCreateRangedPool, MsgLimitOrder, MsgMarketOrder, MsgCancelOrder, MsgCancelAllOrders}, liquidity::{DepositRequest, WithdrawRequest}}};

pub struct ShogunContract;

pub type ContractError = cosmwasm_std::StdError;

#[contract]
impl ShogunContract {
    pub const fn new() -> Self {
        Self{}
    }

    #[msg(instantiate)]
    pub fn instantiate(&self, _ctx: (DepsMut, Env, MessageInfo)) -> StdResult<Response> {
        Ok(Response::new())
    }

    #[msg(exec)]
    pub fn create_pair(
        &self,
        ctx: (DepsMut, Env, MessageInfo),
        msg: MsgCreatePair
    ) -> Result<Response, ContractError> {
        create_pair(ctx, msg)
    }

    #[msg(exec)]
    pub fn create_pool(
        &self,
        ctx: (DepsMut, Env, MessageInfo),
        msg: MsgCreatePool
    ) -> Result<Response, ContractError> {
        create_pool(ctx, msg)
    }

    #[msg(exec)]
    pub fn create_ranged_pool(
        &self,
        ctx: (DepsMut, Env, MessageInfo),
        msg: MsgCreateRangedPool
    ) -> Result<Response, ContractError> {
        create_ranged_pool(ctx, msg)
    }

    #[msg(exec)]
    pub fn deposit(
        &self,
        ctx: (DepsMut, Env, MessageInfo),
        msg: DepositRequest
    ) -> Result<Response, ContractError> {
        deposit(ctx, msg)
    }

    #[msg(exec)]
    pub fn withdraw(
        &self,
        ctx: (DepsMut, Env, MessageInfo),
        msg: WithdrawRequest
    ) -> Result<Response, ContractError> {
        withdraw(ctx, msg)
    }

    #[msg(exec)]
    pub fn limit_order(
        &self,
        ctx: (DepsMut, Env, MessageInfo),
        msg: MsgLimitOrder
    ) -> Result<Response, ContractError> {
        limit_order(ctx, msg)
    }

    #[msg(exec)]
    pub fn market_order(
        &self,
        ctx: (DepsMut, Env, MessageInfo),
        msg: MsgMarketOrder
    ) -> Result<Response, ContractError> {
        market_order(ctx, msg)
    }

    #[msg(exec)]
    pub fn cancel_order(
        &self,
        ctx: (DepsMut, Env, MessageInfo),
        msg: MsgCancelOrder
    ) -> Result<Response, ContractError> {
        cancel_order(ctx, msg)
    }

    #[msg(exec)]
    pub fn cancel_all_orders(
        &self,
        ctx: (DepsMut, Env, MessageInfo),
        msg: MsgCancelAllOrders
    ) -> Result<Response, ContractError> {
        cancel_all_orders(ctx, msg)
    }
}
