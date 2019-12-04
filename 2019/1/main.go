package main

import (
	"fmt"
)

func main() {
	input := numbers()
	fmt.Println(fuelCounterUpper(input))
}

type entry struct {
	key    string
	value  int
	leaves []string
}

func requiredFuel1(n int) int {
	return (n / 3) - 2
}

func requiredFuel(n int) int {
	sum := -n
	for f := n; f > 0; f = requiredFuel1(f) {
		fmt.Println(f)
		sum += f
	}
	return sum
}

func fuelCounterUpper(ns []int) int {
	sum := 0
	for _, n := range ns {
		sum += requiredFuel(n)
	}
	return sum
}
