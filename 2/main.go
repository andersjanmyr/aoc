package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func checksum(numbers []int) int {
	length := len(numbers)
	min, max := 10000, 0
	for i := 0; i < length; i++ {
		if numbers[i] < min {
			min = numbers[i]
		}
		if numbers[i] > max {
			max = numbers[i]
		}
	}
	return max - min
}

func divisible(numbers []int) int {
	length := len(numbers)
	for i := 0; i < length-1; i++ {
		for j := i + 1; j < length; j++ {
			if numbers[i]%numbers[j] == 0 {
				return numbers[i] / numbers[j]
			}
			if numbers[j]%numbers[i] == 0 {
				return numbers[j] / numbers[i]
			}
		}
	}
	return 0
}

func total(matrix [][]int) int {
	s := 0
	for _, n := range matrix {
		s += checksum(n)
	}
	return s
}

func toNumbers(l string) []int {
	ns := strings.Split(l, "\t")
	numbers := make([]int, len(ns))
	for i, n := range ns {
		numbers[i], _ = strconv.Atoi(n)
	}
	return numbers
}

func toMatrix(s string) [][]int {
	lines := strings.Split(s, "\n")
	numbers := make([][]int, len(lines))
	for i, l := range lines {
		numbers[i] = toNumbers(l)
	}
	return numbers
}

func totalDivisible(matrix [][]int) int {
	s := 0
	for _, n := range matrix {
		d := divisible(n)
		fmt.Println(d)
		s += d
	}
	return s
}

func main() {
	bytes, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}
	data := string(bytes)
	matrix := toMatrix(data)
	length := len(bytes) - 1
	numbers := make([]int, length)
	for i := 0; i < length; i++ {
		numbers[i] = int(bytes[i]) - 48
	}
	fmt.Println(total(matrix))
	fmt.Println(totalDivisible(matrix))
}
