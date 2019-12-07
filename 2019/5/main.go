package main

import (
	"fmt"
)

const END = 99

var opMap = map[int]int{}

const (
	Add = iota + 1
	Multiply
	Input
	Output
	JumpIfTrue
	JumpIfFalse
	LessThan
	Equals
	End = 99
)

const (
	Position  = 0
	Immediate = 1
)

func init() {
	opMap[Add] = 3
	opMap[Multiply] = 3
	opMap[Input] = 1
	opMap[Output] = 1
	opMap[JumpIfTrue] = 2
	opMap[JumpIfFalse] = 2
	opMap[LessThan] = 3
	opMap[Equals] = 3
}

type Inst struct {
	op int
}

func main() {
	input := numbersFromCSV()
	fmt.Println(input)
	ns, out := intCode(input, 5)
	fmt.Println(ns)
	fmt.Println(out)
}

func intCode(ns []int, input int) ([]int, int) {
	i := 0
	out := 0
	os := parse(ns[i])
	op := at(os, 0)
	for op != END {
		// fmt.Println("op", op)
		jump := 0
		switch op {
		case Add:
			r := read(ns, i+1, at(os, 1)) + read(ns, i+2, at(os, 2))
			write(ns, i+3, 0, r)
		case Multiply:
			r := read(ns, i+1, at(os, 1)) * read(ns, i+2, at(os, 2))
			write(ns, i+3, 0, r)
		case Input:
			write(ns, i+1, at(os, 1), input)
		case Output:
			r := read(ns, i+1, at(os, 1))
			fmt.Println("output:", r)
			out = r
		case JumpIfTrue:
			j := read(ns, i+1, at(os, 1))
			if j != 0 {
				jump = read(ns, i+2, at(os, 2))
			}
		case JumpIfFalse:
			j := read(ns, i+1, at(os, 1))
			if j == 0 {
				jump = read(ns, i+2, at(os, 2))
			}
		case LessThan:
			a := read(ns, i+1, at(os, 1))
			b := read(ns, i+2, at(os, 2))
			v := 0
			if a < b {
				v = 1
			}
			write(ns, i+3, at(os, 3), v)
		case Equals:
			a := read(ns, i+1, at(os, 1))
			b := read(ns, i+2, at(os, 2))
			v := 0
			if a == b {
				v = 1
			}
			write(ns, i+3, at(os, 3), v)
		default:
			panic(op)
		}
		if jump == 0 {
			i += opMap[op] + 1
		} else {
			i = jump
		}
		os = parse(ns[i])
		op = at(os, 0)
	}
	return ns, out
}

func parse(i int) []int {
	is := []int{}
	for i > 0 {
		is = append(is, i%10)
		i /= 10
	}
	op := at(is, 0) + 10*at(is, 1)
	if len(is) == 1 {
		is = append(is, op)
	} else {
		is[1] = op
	}
	return is[1:]
}

func at(is []int, i int) int {
	if i >= len(is) {
		return 0
	}
	return is[i]
}

func read(ns []int, index, mode int) int {
	i := ns[index]
	if mode == Position {
		return ns[i]
	}
	return i
}

func write(ns []int, index, mode, val int) {
	i := ns[index]
	if mode == Position {
		ns[i] = val
		return
	}
	panic("Never Immediate")
}
