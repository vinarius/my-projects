package card

import (
	"fmt"
	"strings"

	"github.com/fatih/color"
)

func (r Rank) String() string {
	switch r {
	case Two:
		return "2"
	case Three:
		return "3"
	case Four:
		return "4"
	case Five:
		return "5"
	case Six:
		return "6"
	case Seven:
		return "7"
	case Eight:
		return "8"
	case Nine:
		return "9"
	case Ten:
		return "10"
	case Jack:
		return "J"
	case Queen:
		return "Q"
	case King:
		return "K"
	case Ace:
		return "A"
	default:
		return "?"
	}
}

func (c Card) String() string {
	rankStr := c.Rank.String()
	rankStrFirst := c.Rank.String()
	suitStr := c.Suit.String()
	suitStrFirst := " " + c.Suit.String()

	if c.Rank == Ten {
		rankStrFirst = " " + rankStrFirst
		suitStrFirst = suitStrFirst[1:]
	} else {
		rankStr = " " + rankStr
		rankStrFirst = " " + rankStrFirst
	}

	cardStr := fmt.Sprintf(
		"┌───────┐\n"+
			"│%s%s   │\n"+
			"│       │\n"+
			"│   %s   │\n"+
			"│       │\n"+
			"│   %s%s │\n"+
			"└───────┘",
		rankStrFirst, suitStrFirst, suitStr, suitStr, rankStr,
	)

	return cardStr
}

func (s Suit) String() string {
	switch s {
	case Spades:
		return "♠"
	case Hearts:
		return "♥"
	case Clubs:
		return "♣"
	case Diamonds:
		return "♦"
	default:
		return "?"
	}
}

type (
	Suit int
	Rank int
)

const (
	Spades Suit = iota
	Hearts
	Clubs
	Diamonds
)

const (
	Two Rank = iota
	Three
	Four
	Five
	Six
	Seven
	Eight
	Nine
	Ten
	Jack
	Queen
	King
	Ace
)

type Card struct {
	Rank Rank
	Suit Suit
}

func Print(cards []Card, printOnSameRow bool) {
	if printOnSameRow {
		if len(cards) == 0 {
			return
		}

		cardStrings := make([][]string, len(cards))
		for i, card := range cards {
			cardStrings[i] = strings.Split(card.String(), "\n")
		}

		for lineNum := range cardStrings[0] {
			for cardNum, card := range cards {
				line := cardStrings[cardNum][lineNum]

				if card.Suit == Hearts || card.Suit == Diamonds {
					fmt.Print(color.RedString(line))
				} else {
					fmt.Print(line)
				}
			}
			fmt.Println()
		}

		return
	}

	for _, card := range cards {
		cardStr := card.String()
		if card.Suit == Hearts || card.Suit == Diamonds {
			fmt.Println(color.RedString(cardStr))
		} else {
			fmt.Println(cardStr)
		}
	}
}
