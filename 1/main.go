package main

import (
	"fmt"
	"io/ioutil"
)

func consecutives(numbers []int) []int {
	length := len(numbers)
	cons := make([]int, 0)
	found := false
	for i := 0; i < length-1; i++ {
		if numbers[i] == numbers[i+1] {
			cons = append(cons, numbers[i])
			found = true
		} else if found {
			cons = append(cons, numbers[i])
			found = false
		}
	}
	if numbers[0] == numbers[length-1] {
		if numbers[0] != numbers[1] {
			cons = append(cons, numbers[0])
		}
		if numbers[length-2] != numbers[length-1] {
			cons = append(cons, numbers[length-1])
		}
	}
	return cons
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
	cons := consecutives(numbers)
	fmt.Println(cons)
	sum := 0
	for _, n := range cons {
		sum += n
	}
	fmt.Println(sum)
}
