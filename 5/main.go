package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	input := numbers()
	fmt.Println(input)
	steps := exit(input)
	fmt.Println(steps)
}

func numbers() []int {
	bytes, _ := ioutil.ReadFile("input.txt")
	data := string(bytes)
	ss := strings.Split(data, "\n")
	ns := make([]int, len(ss)-1)
	for i := 0; i < len(ss)-1; i++ {
		ns[i], _ = strconv.Atoi(ss[i])
	}
	return ns
}

func exit(input []int) int {
	i := 0
	steps := 0
	l := len(input)
	for i < l {
		steps++
		i = step(input, i)
	}
	return steps
}

func step(input []int, i int) int {
	j := i + input[i]
	if input[i] >= 3 {
		input[i]--
	} else {
		input[i]++
	}
	return j
}
