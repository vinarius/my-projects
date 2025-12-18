package game

import (
	"fmt"
	"slices"

	"cli-poker/models/deck"
	"cli-poker/models/player"
)

type Game struct {
	Players *[]player.Player
	Deck    *deck.Deck
}

func New() *Game {
	deck := deck.New()
	deck.Shuffle()

	return &Game{
		Players: &[]player.Player{},
		Deck:    &deck,
	}
}

func (game *Game) AddPlayer(newPlayer *player.Player) error {
	indexOfPlayerAlreadyUsingUsername := slices.IndexFunc(*game.Players, func(gamePlayer player.Player) bool {
		return gamePlayer.Username == newPlayer.Username
	})

	if indexOfPlayerAlreadyUsingUsername != -1 {
		return fmt.Errorf("Unable to add player. Username \"%v\" is already in use\n", newPlayer.Username)
	}

	*game.Players = append(*game.Players, *newPlayer)

	return nil
}

func (game *Game) RemovePlayer(username string) {
	playerIndex := slices.IndexFunc(*game.Players, func(gamePlayer player.Player) bool {
		return gamePlayer.Username == &username
	})

	if playerIndex == -1 {
		return
	}

	*game.Players = slices.Delete(*game.Players, playerIndex, playerIndex+1)
}

func (game *Game) Deal() {
	game.Deck.Shuffle()

	for i := range *game.Players {
		player := &(*game.Players)[i]
		player.Hand = game.Deck.DealHand()
	}
}
