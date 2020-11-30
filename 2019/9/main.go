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
	AdjustBase
	End = 99
)

const (
	Position  = 0
	Immediate = 1
	Relative  = 2
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
	opMap[AdjustBase] = 1
}

type Inst struct {
	op int
}

func main() {
	input := numbersFromCSV()
	fmt.Println(input)
	o := make(chan int, 10)
	i := make(chan int, 10)
	var r []int
	go func() {
		defer close(o)
		defer close(i)
		r = intCode(input, i, o)
	}()
	i <- 1
	for i := range o {
		fmt.Println("out", i)
	}
	r[0] = 0
}

var num = 0

func intCode(nns []int, input, output chan int) []int {
	ns := map[int]int{}
	for i, n := range nns {
		ns[i] = n
	}
	num++
	relativeBase := 0

	read := func(index, mode int) int {
		i := ns[index]
		if mode == Position {
			// fmt.Println("position", ns[i])
			return ns[i]
		} else if mode == Relative {
			// fmt.Println("relative", ns[relativeBase+i])
			return ns[relativeBase+i]
		}
		// fmt.Println("Immediate", i)
		return i
	}

	write := func(index, mode, val int) {
		i := ns[index]
		if mode == Position {
			ns[i] = val
			// fmt.Println("write position", i, val)
			return
		} else if mode == Relative {
			ns[relativeBase+i] = val
			// fmt.Println("write relative", i, relativeBase, ns[relativeBase+i], val)
			return
		}
		panic("Never Immediate")
	}

	i := 0
	os := parse(ns[i])
	op := at(os, 0)
	for op != END {
		// fmt.Println("op", op)
		jump := -1111
		switch op {
		case Add:
			r := read(i+1, at(os, 1)) + read(i+2, at(os, 2))
			write(i+3, 0, r)
		case Multiply:
			r := read(i+1, at(os, 1)) * read(i+2, at(os, 2))
			write(i+3, 0, r)
		case Input:
			// fmt.Println("before read", num)
			in := <-input
			// fmt.Println("read", in, num)
			write(i+1, at(os, 1), in)
		case Output:
			r := read(i+1, at(os, 1))
			fmt.Println("before write", r)
			output <- r
		case JumpIfTrue:
			j := read(i+1, at(os, 1))
			if j != 0 {
				jump = read(i+2, at(os, 2))
			}
		case JumpIfFalse:
			j := read(i+1, at(os, 1))
			if j == 0 {
				jump = read(i+2, at(os, 2))
			}
		case LessThan:
			a := read(i+1, at(os, 1))
			b := read(i+2, at(os, 2))
			v := 0
			if a < b {
				v = 1
			}
			write(i+3, at(os, 3), v)
		case Equals:
			a := read(i+1, at(os, 1))
			b := read(i+2, at(os, 2))
			v := 0
			if a == b {
				v = 1
			}
			write(i+3, at(os, 3), v)
		case AdjustBase:
			a := read(i+1, at(os, 1))
			relativeBase += a
			fmt.Println("rel", relativeBase, a)
		default:
			panic(op)
		}
		if jump == -1111 {
			i += opMap[op] + 1
		} else {
			i = jump
		}
		os = parse(ns[i])
		op = at(os, 0)
	}
	nns = []int{}
	for i := 0; i < len(ns); i++ {
		nns = append(nns, ns[i])
	}
	return nns
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
