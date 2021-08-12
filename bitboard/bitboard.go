package bitboard

import (
	"github.com/holiman/uint256"
)

type Bitboard *uint256.Int

func InitEmptyBB() Bitboard{
	return uint256.NewInt(0)
}

func PosToIndex(x,y int) int{
	return (16*y + x)
}

func IndexToPos(index int) (int, int){
	return index%16, index/16
}