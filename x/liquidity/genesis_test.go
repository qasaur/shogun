package liquidity_test

import (
	"testing"

	"github.com/stretchr/testify/require"
	keepertest "shogun/testutil/keeper"
	"shogun/testutil/nullify"
	"shogun/x/liquidity"
	"shogun/x/liquidity/types"
)

func TestGenesis(t *testing.T) {
	genesisState := types.GenesisState{
		Params: types.DefaultParams(),

		// this line is used by starport scaffolding # genesis/test/state
	}

	k, ctx := keepertest.LiquidityKeeper(t)
	liquidity.InitGenesis(ctx, *k, genesisState)
	got := liquidity.ExportGenesis(ctx, *k)
	require.NotNil(t, got)

	nullify.Fill(&genesisState)
	nullify.Fill(got)

	// this line is used by starport scaffolding # genesis/test/assert
}
