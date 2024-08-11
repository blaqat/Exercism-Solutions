package blackjack

// ParseCard returns the integer value of a card following blackjack ruleset.
func ParseCard(card string) int {
	switch card {
	case "two":
		return 2
	case "three":
		return 3
	case "four":
		return 4
	case "five":
		return 5
	case "six":
		return 6
	case "seven":
		return 7
	case "eight":
		return 8
	case "nine":
		return 9
	case "ten", "jack", "queen", "king":
		return 10
	case "ace":
		return 11
	default:
		return 0
	}
}

// FirstTurn returns the decision for the first turn, given two cards of the
// player and one card of the dealer.
func FirstTurn(card1, card2, dealerCard string) string {
	const (
		Stand = "S"
		Hit   = "H"
		Split = "P"
		Win   = "W"
	)

	val1, val2, dealerVal := ParseCard(card1), ParseCard(card2), ParseCard(dealerCard)
	sum := val1 + val2

	switch {
	// Split
	case val1 == val2 && val1 == 11:
		return Split

	// Win
	case sum == 21 && dealerVal < 10:
		return Win

	// Stand
	case sum == 21,
		17 <= sum && sum <= 20,
		12 <= sum && sum <= 16 && dealerVal < 7:
		return Stand

	// Hit
	default:
		return Hit
	}
}
