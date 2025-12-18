package deck

import (
	"math/rand"
	"slices"

	"cli-poker/models/card"
	"cli-poker/models/hand"
)

type Deck struct {
	DrawPile       []card.Card
	BurnPile       []card.Card
	CommunityCards []card.Card
}

func New() Deck {
	faceDownCards := []card.Card{}

	for suit := range card.Diamonds + 1 {
		for rank := range card.Ace + 1 {
			faceDownCards = append(faceDownCards, card.Card{Rank: rank, Suit: suit})
		}
	}

	return Deck{
		DrawPile:       faceDownCards,
		BurnPile:       []card.Card{},
		CommunityCards: []card.Card{},
	}
}

func (deck *Deck) Shuffle() {
	tempCards := []card.Card{}
	tempCards = append(tempCards, deck.DrawPile...)
	tempCards = append(tempCards, deck.BurnPile...)
	shuffledCards := []card.Card{}
	deck.BurnPile = []card.Card{}
	deck.DrawPile = []card.Card{}

	for range 10 {
		for i := len(tempCards); i > 0; i-- {
			randomCardIndex := rand.Intn(i)
			randomCardPulled := tempCards[randomCardIndex]
			tempCards = slices.Delete(tempCards, randomCardIndex, randomCardIndex+1)
			shuffledCards = append(shuffledCards, randomCardPulled)
		}

		tempCards = append(tempCards, shuffledCards...)
		shuffledCards = []card.Card{}
	}

	shuffledCards = append(shuffledCards, tempCards...)

	deck.DrawPile = shuffledCards
}

func (deck *Deck) Burn() {
	cardDraw := deck.Draw()
	deck.BurnPile = append(deck.BurnPile, cardDraw)
}

func (deck *Deck) Draw() card.Card {
	if len(deck.DrawPile) == 0 {
		panic("TODO: handle attempting to draw when deck is empty")
	}

	lastIndex := len(deck.DrawPile) - 1
	topCard := deck.DrawPile[lastIndex]
	deck.DrawPile = slices.Delete(deck.DrawPile, lastIndex, lastIndex+1)

	return topCard
}

func (deck *Deck) DealHand() *hand.Hand {
	return &hand.Hand{
		C1: deck.Draw(),
		C2: deck.Draw(),
	}
}

func (deck *Deck) RevealFlop() {
	deck.Burn()

	theFlop := []card.Card{
		deck.Draw(),
		deck.Draw(),
		deck.Draw(),
	}

	deck.CommunityCards = append(deck.CommunityCards, theFlop...)
}

func (deck *Deck) RevealTurn() {
	deck.Burn()

	theTurn := deck.Draw()

	deck.CommunityCards = append(deck.CommunityCards, theTurn)
}

func (deck *Deck) RevealRiver() {
	deck.Burn()

	theRiver := deck.Draw()

	deck.CommunityCards = append(deck.CommunityCards, theRiver)
}
