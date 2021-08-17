package position

import (
	"testing"
	//"fmt"
	//"github.com/holiman/uint256"
	"stonkfish/bitboard"
)

func TestPositionDimensionCount(t *testing.T){
	position:=LoadPositionfromFEN("rnbqkbnrrrrrrrrr/pppppppprrrrrrrr/16/16/16/16/PPPPPPPPNNNNNNNN/RNBQKBNRNNNNNNNN/16/16/16/16/16/16/16/16 w KQkq - 0 1")
	bitboard.DisplayBitboard(position.Pieces[1].Rook.Bitboard)
}
func TestBitOperations(t *testing.T){
	n := bitboard.InitEmptyBB()
	n = setBit(n,0)
	n = clearBit(n,0)
	n = setBit(n,255)
}

