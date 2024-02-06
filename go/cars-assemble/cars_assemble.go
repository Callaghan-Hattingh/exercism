package cars

// import "fmt"


// CalculateWorkingCarsPerHour calculates how many working cars are
// produced by the assembly line every hour.
func CalculateWorkingCarsPerHour(productionRate int, successRate float64) float64 {
	// panic("CalculateWorkingCarsPerHour not implemented")
	fpr := float64(productionRate)
	// fmt.Println(successRate, fpr, fpr * successRate)
	return fpr * successRate / 100
}

// CalculateWorkingCarsPerMinute calculates how many working cars are
// produced by the assembly line every minute.
func CalculateWorkingCarsPerMinute(productionRate int, successRate float64) int {
	// panic("CalculateWorkingCarsPerMinute not implemented")
	fpr := float64(productionRate) 
	return int(fpr * successRate / 60.0 / 100.0) 
}

// CalculateCost works out the cost of producing the given number of cars.
func CalculateCost(carsCount int) uint {
	// panic("CalculateCost not implemented")
	ten := carsCount / 10 
	one := carsCount % 10
	// fmt.Println(ten, one, uint(ten * 95_000 + one * 10_000))
	return uint(ten * 95_000 + one * 10_000)
}
