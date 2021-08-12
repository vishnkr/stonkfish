package position

import (
	"stonkfish/bitboard"
)

type PieceType uint8
const (
	Custom PieceType = iota
	Pawn
	Knight
	Bishop
	Rook
	Queen
	King
)

type Piece struct {
	PieceType PieceType
	Bitboard bitboard.Bitboard
	PieceRepr string
	Player int
}

type PieceSet struct {
	King *Piece
	Queen *Piece
	Rook *Piece
	Pawn *Piece
	Knight *Piece
	Bishop *Piece
	Custom []*Piece
	Player int
}

func NewPieceSet(player int) *PieceSet {
	return &PieceSet{
		Player:player,
		King: InitKing(player),
		Queen: InitQueen(player),
		Rook: InitRook(player),
		Pawn: InitPawn(player),
		Knight: InitKnight(player),
		Bishop: InitBishop(player),
	}
}

func InitPawn(player int) *Piece{
	return &Piece{PieceType:Pawn,PieceRepr:"p",Player:player,Bitboard:bitboard.InitEmptyBB()}
}

func InitBishop(player int) *Piece{
	return &Piece{PieceType:Bishop,PieceRepr:"b",Player:player,Bitboard:bitboard.InitEmptyBB()}
}

func InitQueen(player int) *Piece{
	return &Piece{PieceType:Queen,PieceRepr:"q",Player:player,Bitboard:bitboard.InitEmptyBB()}
}

func InitRook(player int) *Piece{
	return &Piece{PieceType:Rook,PieceRepr:"r",Player:player,Bitboard:bitboard.InitEmptyBB()}
}

func InitKing(player int) *Piece{
	return &Piece{PieceType:King,PieceRepr:"k",Player:player,Bitboard:bitboard.InitEmptyBB()}
}

func InitKnight(player int) *Piece{
	return &Piece{PieceType:Knight,PieceRepr:"n",Player:player,Bitboard:bitboard.InitEmptyBB()}
}

func InitCustom(player int,piece_repr string) *Piece{
	return &Piece{PieceType:Custom,PieceRepr:piece_repr,Player:player,Bitboard:bitboard.InitEmptyBB()}
}