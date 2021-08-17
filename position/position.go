package position

import (
	"github.com/holiman/uint256"
	"stonkfish/bitboard"
	"strconv"
	//"fmt"
	"strings"
	"unicode"
)

type Position struct {
	Turn   int        // player number
	Width  int        //columns
	Height int        //rows
	Pieces []*PieceSet //white and black piece sets
}

func setBit(n *uint256.Int, pos uint) bitboard.Bitboard {
	lshift := uint256.NewInt(1)
	lshift.Lsh(lshift, pos) // n |= (1 << pos)
	n.Or(n, lshift)
	return n
}

func clearBit(n *uint256.Int, pos uint) bitboard.Bitboard {
	mask := uint256.NewInt(1)
	mask.Lsh(mask, pos).Not(mask) // ^(1 << pos)
	n.And(n, mask)                //n &= mask
	return n
}

func LoadPositionfromFEN(fen string) *Position {
	var position *Position = &Position{}
	position.Height, position.Width = getBoardDimensions(fen)
	//var wLeftCastle, wRightCastle, bLeftCastle, bRightCastle bool = false,false,false,false
	wPieceSet, bPieceSet := NewPieceSet(0), NewPieceSet(1)
	//boardData := strings.Split(fen, " ")
	//rowsData := strings.Split(boardData[0], "/")
	var row, col, fenPart, secDigit int = 0, 0, 0, 0
	for index, char := range fen {
		if string(char) == " " {
			fenPart += 1
			continue
		}
		switch fenPart {
		case 0: //board info
			if string(char) == "/" {
				col = 0
				row += 1
				secDigit = 0
				continue
			} else if unicode.IsNumber(rune(char)) {
				//if a number is found check if its a 2-digit number since board dims can exced 8x8
				if (index+1 < position.Width) && unicode.IsNumber(rune(fen[index+1])) {
					secDigit, _ = strconv.Atoi(string(char))
				} else {
					count, _ := strconv.Atoi(string(char))
					if secDigit != 0 {
						col += secDigit*10 + count
						secDigit = 0
					} else {
						col += count
					}
				}
				continue
			}
			var bbIndex = bitboard.PosToIndex(row, col)
			var bitboard bitboard.Bitboard
			var pieceSet *PieceSet
			if unicode.IsUpper(rune(char)) {
				pieceSet = wPieceSet
			} else {
				pieceSet = bPieceSet
			}
			switch strings.ToLower(string(char)) {
			case "k":
				bitboard = pieceSet.King.Bitboard
			case "n":
				bitboard = pieceSet.Knight.Bitboard
			case "b":
				bitboard = pieceSet.Bishop.Bitboard
			case "r":
				bitboard = pieceSet.Rook.Bitboard
			case "q":
				bitboard = pieceSet.Queen.Bitboard
			case "p":
				bitboard = pieceSet.Pawn.Bitboard
			}
			bitboard = setBit(bitboard,uint(bbIndex))
			col+=1
		case 1:
			if string(char) == "w" {
				position.Turn = 0
			} else {
				position.Turn = 1
			}
		}
	}
	position.Pieces = []*PieceSet{wPieceSet,bPieceSet}
	return position
}

func getBoardDimensions(fen string) (int, int) {
	boardData := strings.Split(fen, " ")
	rowsData := strings.Split(boardData[0], "/")
	var colCount int = 0
	var secDigit int = 0
	for index, char := range rowsData[0] {
		if (unicode.IsNumber(rune(char))) {
			count, _ := strconv.Atoi(string(char))
			if index+1 < len(rowsData[0]) && unicode.IsNumber(rune(rowsData[0][index+1])) {
				secDigit, _ = strconv.Atoi(string(char))
			} else {
				if secDigit != 0 {
					colCount += secDigit*10 + count
					secDigit = 0
				} else {
					colCount += count
				}
			}
		} else {
			colCount += 1
		}
	}
	return len(rowsData), colCount
}
