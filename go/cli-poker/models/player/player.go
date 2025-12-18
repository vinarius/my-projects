package player

import (
	"cli-poker/models/hand"
)

type (
	Player struct {
		Username *string
		Hand     *hand.Hand
		Cash     *float64
		IsUser   *bool
	}
	NewPlayerInput struct {
		Username string
		IsUser   bool
	}
)

func New(input *NewPlayerInput) *Player {
	cash := 500.00

	return &Player{
		Username: &input.Username,
		Hand:     &hand.Hand{},
		Cash:     &cash,
		IsUser:   &input.IsUser,
	}
}
