// Package weather lib.
package weather


// CurrentCondition var.
var CurrentCondition string 
// CurrentLocation var.
var CurrentLocation string 

// Forecast fn.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
