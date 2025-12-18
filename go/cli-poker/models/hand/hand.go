package hand

import (
	"slices"

	"cli-poker/models/card"
)

type Hand struct {
	C1 card.Card
	C2 card.Card
}

func (hand *Hand) ToSlice() []card.Card {
	return []card.Card{hand.C1, hand.C2}
}

type Rank int

const (
	HighCard Rank = iota
	OnePair
	TwoPair
	ThreeOfAKind
	Straight
	Flush
	FullHouse
	FourOfAKind
	StraightFlush
	RoyalFlush
)

func (rank Rank) String() string {
	switch rank {
	case HighCard:
		return "High Card"
	case OnePair:
		return "One Pair"
	case TwoPair:
		return "Two Pair"
	case ThreeOfAKind:
		return "Three Of A Kind"
	case Straight:
		return "Straight"
	case Flush:
		return "Flush"
	case FullHouse:
		return "Full House"
	case FourOfAKind:
		return "Four Of A Kind"
	case StraightFlush:
		return "Straight Flush"
	case RoyalFlush:
		return "Royal Flush"
	default:
		return "?"
	}
}

type SuitCounter map[card.Suit]int

func (hand *Hand) Evaluate(communityCards []card.Card) Rank {
	// TODO:

	suitCounter := make(SuitCounter)
	rankCounter := make(map[card.Rank]int)
	allCards := slices.Concat(hand.ToSlice(), communityCards)

	for _, card := range allCards {
		suitCounter[card.Suit]++
		rankCounter[card.Rank]++
	}

	isRoyalFlush(&allCards)

	// hasFiveOrMoreOfSameSuit := hasFiveOrMoreOfSameSuit(suitCounter)

	return HighCard
}

// func hasFiveOrMoreOfSameSuit(suitCounter SuitCounter) bool {
// 	for _, value := range suitCounter {
// 		if value >= 5 {
// 			return true
// 		}
// 	}

// 	return false
// }

func isRoyalFlush(allCards *[]card.Card) bool {
	// var hasTen, hasJack, hasQueen, hasKing, hasAce bool

	// make a slice of cards
	// iterate over community cards and hand
	// if 10, push, if jack, push, etc

	// royalFlushCandidates := make([]card.Card, 5)

	card.Print(*allCards, true)

	return true
}
