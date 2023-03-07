package keeper_test

import (
	"testing"

	"github.com/stretchr/testify/require"
	testkeeper "shogun/testutil/keeper"
	"shogun/x/liquidity/types"
)

func TestGetParams(t *testing.T) {
	k, ctx := testkeeper.LiquidityKeeper(t)
	params := types.DefaultParams()

	k.SetParams(ctx, params)

	require.EqualValues(t, params, k.GetParams(ctx))
}
