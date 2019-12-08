package main

import (
	"fmt"
	"sync"
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
	// ns, out := intCode(input, []int{5})
	ns, out := maxAmplifiedSignal(input)
	fmt.Println(ns, out)
}

var num = 0

func intCode(ns []int, input, output chan int) []int {
	num++
	fmt.Println(input, output, num)
	i := 0
	os := parse(ns[i])
	op := at(os, 0)
	for op != END {
		jump := 0
		switch op {
		case Add:
			r := read(ns, i+1, at(os, 1)) + read(ns, i+2, at(os, 2))
			write(ns, i+3, 0, r)
		case Multiply:
			r := read(ns, i+1, at(os, 1)) * read(ns, i+2, at(os, 2))
			write(ns, i+3, 0, r)
		case Input:
			fmt.Println("before read", num)
			in := <-input
			fmt.Println("read", in, num)
			write(ns, i+1, at(os, 1), in)
		case Output:
			r := read(ns, i+1, at(os, 1))
			fmt.Println("before write", num)
			output <- r
			fmt.Println("write", r, num)
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
	return ns
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

func maxThrustSignal(ns []int) (int, int) {
	max := 0
	seq := 0
	for _, c := range combinations(0, 5) {
		ee := thrustSignal(ns, c)
		if ee > max {
			max = ee
			seq = toInt(c)
		}
	}
	return max, seq
}

func maxAmplifiedSignal(ns []int) (int, int) {
	max := 0
	seq := 0
	for _, c := range combinations(5, 10) {
		ee := amplifiedSignal(ns, c)
		if ee > max {
			max = ee
			seq = toInt(c)
		}
	}
	return max, seq
}

func combinations(min, max int) [][]int {
	cs := [][]int{}
	for a := min; a < max; a++ {
		for b := min; b < max; b++ {
			for c := min; c < max; c++ {
				for d := min; d < max; d++ {
					for e := min; e < max; e++ {
						m := map[int]int{}
						for i := min; i < max; i++ {
							m[i] = i
						}
						delete(m, a)
						delete(m, b)
						delete(m, c)
						delete(m, d)
						delete(m, e)
						if len(m) == 0 {
							cs = append(cs, []int{a, b, c, d, e})
						}
					}
				}
			}
		}
	}
	return cs
}

func thrustSignal(ns []int, is []int) int {
	i := make(chan int, 100)
	var o chan int
	ii := i
	var wg sync.WaitGroup
	for _, n := range is {
		wg.Add(1)
		o = make(chan int, 100)
		copy := append([]int{}, ns...)
		go func(copy []int, i, o chan int) {
			defer close(o)
			intCode(copy, i, o)
			wg.Done()
		}(copy, i, o)
		i <- n
		i = o
	}
	defer close(ii)
	ii <- 0
	wg.Wait()
	return <-o
}

func amplifiedSignal(ns []int, is []int) int {
	i := make(chan int, 100)
	var o chan int
	ii := i
	var wg sync.WaitGroup
	for index, n := range is {
		wg.Add(1)
		if index == 4 {
			fmt.Println("loop")
			o = ii
		} else {
			o = make(chan int, 100)
		}
		copy := append([]int{}, ns...)
		go func(copy []int, i, o chan int) {
			intCode(copy, i, o)
			wg.Done()
		}(copy, i, o)
		i <- n
		i = o
	}
	ii <- 0
	wg.Wait()
	return <-o
}

func toInt(is []int) int {
	return is[0]*10000 + is[1]*1000 + is[2]*100 + is[3]*10 + is[4]
}
