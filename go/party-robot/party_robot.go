package partyrobot

import "fmt"

// Welcome greets a person by name.
func Welcome(name string) string {
	//panic("Please implement the Welcome function")
	return fmt.Sprintf("Welcome to my party, %s!", name)
}

// HappyBirthday wishes happy birthday to the birthday person and exclaims their age.
func HappyBirthday(name string, age int) string {
	//panic("Please implement the HappyBirthday function")
	return fmt.Sprintf("Happy birthday %s! You are now %d years old!", name, age)
}

// AssignTable assigns a table to each guest.
func AssignTable(name string, table int, neighbor, direction string, distance float64) string {
	//panic("Please implement the AssignTable function")
	r_name := Welcome(name)
	r_table := fmt.Sprintf("\nYou have been assigned to table %03d.", table)
	r_dir := fmt.Sprintf(" Your table is %s, exactly %.1f meters from here.\n", direction, distance)
	r_neighbor := fmt.Sprintf("You will be sitting next to %s.", neighbor)
	return r_name + r_table + r_dir + r_neighbor
}
