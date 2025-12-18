package main

import (
	"fmt"

	"cli-poker/models/card"
	"cli-poker/models/game"
	"cli-poker/models/player"
)

func main() {
	g := game.New()

	player1 := player.New(&player.NewPlayerInput{
		Username: "vin",
		IsUser:   true,
	})

	g.AddPlayer(player1)

	botCount := 0

	for i := range botCount {
		player := player.New(&player.NewPlayerInput{
			Username: fmt.Sprintf("bot%v", i+1),
		})

		g.AddPlayer(player)
	}

	// TODO: before game starts, let players enter or leave game
	// TODO: big blind, small blind

	g.Deal()

	for _, player := range *g.Players {
		fmt.Println(*player.Username)
		card.Print(player.Hand.ToSlice(), true)
		fmt.Println()
	}

	g.Deck.RevealFlop()
	g.Deck.RevealRiver()
	g.Deck.RevealTurn()

	// (*g.Players)[0].Hand.Evaluate(g.Deck.CommunityCards)

	fmt.Println("Community cards:")
	card.Print(g.Deck.CommunityCards, true)
}
