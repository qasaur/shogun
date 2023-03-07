use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{types::{tx::MsgCreatePair, liquidity::Pair}, store::{get_last_pair_id, set_last_pair_id, set_pair, set_pair_index, set_pair_lookup_index}, contract::ContractError};

pub fn create_pair(ctx: (DepsMut, Env, MessageInfo), msg: MsgCreatePair) -> Result<Response, ContractError> {
    // TODO: validate message
    let id = get_last_pair_id(ctx) + 1;
    set_last_pair_id(ctx, id);

    let pair = Pair::new();
    pair.id = id;
    pair.base_coin_denom = msg.base_coin_denom;
    pair.quote_coin_denom = msg.quote_coin_denom;

    set_pair(ctx, pair);
    set_pair_index(ctx, msg.base_coin_denom, msg.quote_coin_denom, id);
    set_pair_lookup_index(ctx, msg.base_coin_denom, msg.quote_coin_denom, id);
    set_pair_lookup_index(ctx, msg.quote_coin_denom, msg.base_coin_denom, id);

    // TODO: Emit event
    let resp = Response::new();

    Ok(resp)
}