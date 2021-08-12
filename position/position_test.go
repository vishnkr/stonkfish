package position

import (
	"testing"
	"fmt"
	//"github.com/holiman/uint256"
	"stonkfish/bitboard"
)

func TestPositionDimensionCount(t *testing.T){
	fmt.Println(ConverFENtoPosition("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR/8/8/8/8/8 w KQkq - 0 1"))
}
func TestBitOperations(t *testing.T){
	n := bitboard.InitEmptyBB()
	n = setBit(n,0)
	fmt.Println("num",n)
	n = clearBit(n,0)
	n = setBit(n,255)
	fmt.Println("num",*n)
}