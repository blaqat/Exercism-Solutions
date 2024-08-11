package purchase

import "fmt"

// NeedsLicense determines whether a license is needed to drive a type of vehicle. Only "car" and "truck" require a license.
func NeedsLicense(kind string) bool {
	return kind == "car" || kind == "truck"
}

// ChooseVehicle recommends a vehicle for selection. It always recommends the vehicle that comes first in lexicographical order.
func ChooseVehicle(option1, option2 string) string {
	message := "%s is clearly the better choice."

	if option1 < option2 {
		message = fmt.Sprintf(message, option1)
	} else {
		message = fmt.Sprintf(message, option2)
	}

	return message
}

// CalculateResellPrice calculates how much a vehicle can resell for at a certain age.
func CalculateResellPrice(originalPrice, age float64) float64 {
	var discount float64

	if age < 3 {
		discount = 0.80
	} else if age >= 10 {
		discount = 0.50
	} else {
		discount = 0.70
	}

	return originalPrice * discount
}
