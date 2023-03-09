package types_test

import (
	"testing"

	"cosmossdk.io/math"
	"github.com/stretchr/testify/require"

	utils "shogun/types"
	"shogun/x/liquidity/types"
)

func TestMMOrderTicks(t *testing.T) {
	require.Equal(t,
		[]types.MMOrderTick{
			{OfferCoinAmount: math.NewInt(100000), Price: utils.ParseDec("105"), Amount: math.NewInt(100000)},
			{OfferCoinAmount: math.NewInt(100000), Price: utils.ParseDec("104.45"), Amount: math.NewInt(100000)},
			{OfferCoinAmount: math.NewInt(100000), Price: utils.ParseDec("103.89"), Amount: math.NewInt(100000)},
			{OfferCoinAmount: math.NewInt(100000), Price: utils.ParseDec("103.34"), Amount: math.NewInt(100000)},
			{OfferCoinAmount: math.NewInt(100000), Price: utils.ParseDec("102.78"), Amount: math.NewInt(100000)},
			{OfferCoinAmount: math.NewInt(100000), Price: utils.ParseDec("102.23"), Amount: math.NewInt(100000)},
			{OfferCoinAmount: math.NewInt(100000), Price: utils.ParseDec("101.67"), Amount: math.NewInt(100000)},
			{OfferCoinAmount: math.NewInt(100000), Price: utils.ParseDec("101.12"), Amount: math.NewInt(100000)},
			{OfferCoinAmount: math.NewInt(100000), Price: utils.ParseDec("100.56"), Amount: math.NewInt(100000)},
			{OfferCoinAmount: math.NewInt(100000), Price: utils.ParseDec("100"), Amount: math.NewInt(100000)},
		},
		types.MMOrderTicks(
			types.OrderDirectionSell, utils.ParseDec("100"), utils.ParseDec("105"),
			math.NewInt(1000000), types.DefaultMaxNumMarketMakingOrderTicks, 4),
	)

	require.Equal(t,
		[]types.MMOrderTick{
			{
				OfferCoinAmount: math.NewInt(5402),
				Price:           utils.ParseDec("100.02"),
				Amount:          math.NewInt(54),
			},
			{
				OfferCoinAmount: math.NewInt(5502),
				Price:           utils.ParseDec("100.03"),
				Amount:          math.NewInt(55),
			},
		},
		types.MMOrderTicks(
			types.OrderDirectionBuy, utils.ParseDec("100.02"), utils.ParseDec("100.03"),
			math.NewInt(109), types.DefaultMaxNumMarketMakingOrderTicks, 4),
	)
}
