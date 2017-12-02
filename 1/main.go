package main

import (
	"fmt"
	"io/ioutil"
)

func consecutives(numbers []int) []int {
	length := len(numbers)
	cons := make([]int, 0)
	for i := 0; i < length; i++ {
		if numbers[i] == numbers[(i+1)%length] {
			cons = append(cons, numbers[i])
		}
	}
	return cons
}

func sum(numbers []int) int {
	cons := consecutives(numbers)
	fmt.Println(cons)
	s := 0
	for _, n := range cons {
		s += n
	}
	return s
}

func main() {
	bytes, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}
	length := len(bytes) - 1
	numbers := make([]int, length)
	for i := 0; i < length; i++ {
		numbers[i] = int(bytes[i]) - 48
	}
	fmt.Println(numbers)
	fmt.Println(sum(numbers))
}
