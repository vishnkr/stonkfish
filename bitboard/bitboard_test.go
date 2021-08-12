package bitboard


import "testing"
import "fmt"

func TestBitboardConversions(t *testing.T) {
	var bb Bitboard = InitEmptyBB()
	fmt.Print("bb",bb)
}