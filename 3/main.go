package main

import (
	"fmt"
	"math"
)

var coordIndex map[point]int
var coords map[int]point
var values []int

/*
17  16  15  14  13
18   5   4   3  12
19   6   1   2  11
20   7   8   9  10
21  22  23---> ...
*/
func manhattan(n int) int {
	buildIndex(n)
	p := coords[n]
	return int(math.Abs(float64(p.X)) + math.Abs(float64(p.Y)))
}

func calcD(level, x, y, xd, yd int) (int, int) {
	if x == level && xd == 1 {
		return 0, 1
	}
	if y == level && yd == 1 {
		return -1, 0
	}
	if -x == level && xd == -1 {
		return 0, -1
	}
	if -y == level && yd == -1 {
		return 1, 0
	}
	return xd, yd
}

type point struct {
	X, Y int
}

func buildIndex(n int) {
	coordIndex = make(map[point]int)
	coords = make(map[int]point)
	x := 0
	y := 0
	xd := 1
	yd := 0
	level := 1
	count := 8
	i := 1
	for n > 1 {
		i++
		n--
		x += xd
		y += yd
		if i-1 == count {
			level++
			count += level * 8
		}
		coordIndex[point{x, y}] = i
		coords[i] = point{x, y}
		xd, yd = calcD(level, x, y, xd, yd)
	}
	coordIndex[point{0, 0}] = 1
	coords[1] = point{0, 0}
}

func value(p point) int {
	i := coordIndex[p]
	if i >= len(values) {
		return 0
	}
	val := values[i]
	return val
}

func summarize(p point) int {
	neighbors := []point{
		point{p.X + 1, p.Y},
		point{p.X + 1, p.Y + 1},
		point{p.X, p.Y + 1},
		point{p.X - 1, p.Y + 1},
		point{p.X - 1, p.Y},
		point{p.X - 1, p.Y - 1},
		point{p.X, p.Y - 1},
		point{p.X + 1, p.Y - 1},
	}
	s := 0
	for _, n := range neighbors {
		s += value(n)
	}
	return s
}

func squareSum(n int) int {
	values = make([]int, 100000)
	buildIndex(n)
	values[1] = 1
	for i := 2; i <= n; i++ {
		p := coords[i]
		values[i] = summarize(p)
	}
	return values[n]
}

func main() {
	fmt.Println(manhattan(1024))
	fmt.Println(manhattan(347991))
}
