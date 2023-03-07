use contract::{ShogunContract, InstantiateMsg, ContractExecMsg, ContractQueryMsg, ContractError};
use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Empty, StdResult, Response, Binary, Deps};

pub mod contract;
mod types;

pub mod amm;

pub mod batch;
pub mod pair;
pub mod pool;
pub mod store;
pub mod swap;

const CONTRACT: ShogunContract = ShogunContract::new();

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    msg.dispatch(&CONTRACT, (deps, env, info))
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: ContractQueryMsg) -> Result<Binary, ContractError> {
    msg.dispatch(&CONTRACT, (deps, env))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ContractExecMsg,
) -> Result<Response, ContractError> {
    msg.dispatch(&CONTRACT, (deps, env, info))
}