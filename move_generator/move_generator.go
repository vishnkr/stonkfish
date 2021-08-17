package move_generator

import (
	"stonkfish/position"
)

type MoveType uint8

const (
	Quiet MoveType = iota //000
	Promotion //001
	Capture //010
	RightCastle  // 011 avoiding KingSide/Queenside term since starting pos can be anything
	LeftCastle // 100
	PRCapture //101
	None //110
)

type Move struct{
	Info uint32
	PromoteTo string
} 
// Info field bit structure
//   00000000         00000001      001 
// from 0-7 bits     to 7-15 bits   move-type 16-18 bits

type MoveGenerator struct {

}

func InitMove(src uint8,dest uint8, mType MoveType, promoteStr string) Move{
	var move Move = Move{Info:0,PromoteTo:promoteStr}
	move.Info |= uint32(src)|(uint32(dest) << 8)|(uint32(mType)<<16)
	return move
}

func (move *Move) GetSrc() uint8{
	return uint8(move.Info & uint32(255))
}
func (move *Move) GetDest() uint8{
	return uint8((move.Info >> 8)& uint32(255))
}

func (generator *MoveGenerator) GetPseudoLegalMoves(position *position){

}