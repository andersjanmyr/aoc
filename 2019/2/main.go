package main

import (
	"fmt"
)

const END = 99

func main() {
	input := numbersFromCSV()
	fmt.Println(input)
	n, v := compute(input)
	fmt.Println(n*100 + v)
}

func compute(ns []int) (int, int) {
	for i := 0; i < 100; i++ {
		for j := 0; j < 100; j++ {
			copy := append([]int(nil), ns...)
			copy[1] = i
			copy[2] = j
			r := intCode(copy)[0]
			if r == 19690720 {
				return i, j
			}
		}
	}
	panic("eeroor")
}

func intCode(ns []int) []int {
	for i := 0; i < len(ns); i += 4 {
		if ns[i] == END {
			return ns
		}
		r := op(ns[i], ns[ns[i+1]], ns[ns[i+2]])
		ns[ns[i+3]] = r
	}
	return ns
}

func op(o, a, b int) int {
	switch o {
	case 1:
		return a + b
	case 2:
		return a * b
	default:
		panic("help")
	}

}
