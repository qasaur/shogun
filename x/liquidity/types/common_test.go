package types_test

import (
	"cosmossdk.io/math"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/tendermint/tendermint/crypto"
)

var testAddr = sdk.AccAddress(crypto.AddressHash([]byte("test")))

func newInt(i int64) math.Int {
	return math.NewInt(i)
}
