package main

import (
	"fmt"
	"io/ioutil"
)

var regs map[string]int
var maxa int
var rega string

func main() {
	bytes, _ := ioutil.ReadFile("input.txt")
	sum, gc := calcGroups(bytes)
	fmt.Println(sum, gc)
}

func calcGroups(bytes []byte) (int, int) {
	level := 0
	groups := make([]int, 0)
	sum := 0
	i := 0
	garbageCount := 0
	for i < len(bytes) {
		c := bytes[i]
		if c == '<' {
			var gc int
			i, gc = skipGarbage(bytes, i)
			garbageCount += gc
		} else {
			i++
		}
		if c == '{' {
			level++
		}
		if c == '}' {
			groups = append(groups, level)
			sum += level
			level--
		}
	}
	return sum, garbageCount
}

func skipGarbage(bytes []byte, pos int) (int, int) {
	c := bytes[pos]
	count := 0
	for c != '>' {
		if c == '!' {
			pos += 2
		} else {
			pos++
			count++
		}
		c = bytes[pos]
	}
	pos++
	return pos, count - 1
}
