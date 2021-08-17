package bitboard

import (
	"github.com/holiman/uint256"
	"fmt"
)

type Bitboard = *uint256.Int

func InitEmptyBB() Bitboard{
	return uint256.NewInt(0)
}

func PosToIndex(x,y int) int{
	return (16*x + y)
}

func IndexToPos(index int) (int, int){
	return index%16, index/16
}
func IsBitSet(bitboard Bitboard, index int) bool{
	var largeOne = uint256.NewInt(1)
	//n & (1 << (k))
	bit:=largeOne.And(bitboard,largeOne.Lsh(largeOne,uint(index)))
	return !bit.IsZero()
}

func DisplayBitboard(bitboard Bitboard){
	for row:=0; row<16;row+=1{
		for col:=0;col<16;col+=1{
			index:= PosToIndex(row,col)
			if (IsBitSet(bitboard,index)){
				fmt.Print("1 ")
			} else {fmt.Print("0 ")}
			    
		}
		fmt.Print("\n")
	}
}