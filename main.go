package main

import (
	"github.com/vishnkr/stonkfish/position"
)

type Engine struct {
	Position position
}

func NewEngine(fen string) *Engine{
	return &Engine{
		Position: position.LoadPositionfromFEN(fen)
	}
}
/*

//based off https://www.chessprogramming.org/Perft
func Perft(depth uint8) uint64{
	var nodes uint64
	var moveList //stores movelist from generator
	if depth==0{
		return
	} 
	//generate legal/pseudolegal moves here
	for i:=0;i<len(moveList);i++{
		//psudeolegal? if legal move then move move movelist[i]
		nodes+=Perft(depth-1)
		//undomove
	}
	return nodes
}*/

func main(){
	curPosFen:= "8/5p1k/8/p2p1P2/5q2/P1PbN2p/7P/2Q3K1 w - - 1 44"
	engine := NewEngine(curPosFen)
}