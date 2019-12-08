package main

import (
	"io/ioutil"
	"strconv"
	"strings"
)

func numbersFromCSV() []int {
	ss := lines()
	return toNumbers(strings.Split(ss[0], ","))
}
func numbers() []int {
	ss := lines()
	return toNumbers(ss)
}

func lines() []string {
	bytes, _ := ioutil.ReadFile("input.txt")
	data := string(bytes)
	ss := strings.Split(data, "\n")
	return ss[:len(ss)-1]
}

func toNumbers(ss []string) []int {
	ns := make([]int, len(ss))
	for i := 0; i < len(ss); i++ {
		ns[i], _ = strconv.Atoi(ss[i])
	}
	return ns
}

func toMatrix(ss []string) [][]int {
	nss := make([][]int, len(ss))
	for i := 0; i < len(ss); i++ {
		sss := strings.Split(ss[i], "\t")
		nss[i] = toNumbers(sss)
	}
	return nss
}
