package main

import (
	"reflect"
)

var oldIndexes [][]int

func reallocate(input []int) int {
	oldIndexes = [][]int{}
	for exists(input) < 0 {
		input2 := reallocate2(input)
		oldIndexes = append(oldIndexes, input)
		input = input2
	}
	return len(oldIndexes) - exists(input)
}

func exists(input []int) int {
	for i := 0; i < len(oldIndexes); i++ {
		if reflect.DeepEqual(input, oldIndexes[i]) {
			return i
		}
	}
	return -1
}

func reallocate2(input []int) []int {
	tmp := make([]int, len(input))
	copy(tmp, input)
	index := indexOfLargest(tmp)
	n := tmp[index]
	tmp[index] = 0
	l := len(input)
	i := (index + 1) % l
	for n > 0 {
		n--
		tmp[i]++
		i = (i + 1) % l
	}
	return tmp
}

func indexOfLargest(input []int) int {
	largest := -1
	index := -1
	for i := 0; i < len(input); i++ {
		if input[i] > largest {
			largest = input[i]
			index = i
		}
	}
	return index
}
