package keeper

import (
	"context"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"shogun/x/liquidity/types"
)

func (k msgServer) CreatePair(goCtx context.Context, msg *types.MsgCreatePair) (*types.MsgCreatePairResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	// TODO: Handling the message
	_ = ctx

	return &types.MsgCreatePairResponse{}, nil
}
