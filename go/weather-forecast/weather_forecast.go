// Package weather provides tools to forecast the current weather
// condition of various cities in Goblinocus.
package weather

// CurrentCondition stores the current weather condition of the most recently forecasted location.
var CurrentCondition string

// CurrentLocation stores the location which the weather condition was most recently forecasted.
var CurrentLocation string

// Forecast returns a string containing the weather forecast for the specified city and condition.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
