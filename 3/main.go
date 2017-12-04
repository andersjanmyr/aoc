package main

import (
	"fmt"
)

/*
17  16  15  14  13
18   5   4   3  12
19   6   1   2  11
20   7   8   9  10
21  22  23---> ...
*/
func manhattan(n int) int {
	if n == 1 {
		return 0
	}
	pow := 8
	count := 1
	var i int
	for i = 1; count+pow < n; i++ {
		count += pow
		pow += 8
	}

	return i + delta(i, n-count)
}

func delta(level int, n int) int {
	v := level - 1
	d := -1
	if v == 0 {
		d = 1
	}
	for n > 1 {
		n--
		v += d
		if v == level || v == 0 {
			d = -d
		}
	}
	return v
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

var coordIndex = make(map[point]int)
var coords = make([]point, 100)

func buildIndex(n int) {
	coordIndex = make(map[point]int)
	coords = make([]point, 100)
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

func coordinates(n int) (int, int) {
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
		xd, yd = calcD(level, x, y, xd, yd)
	}
	return x, y
}

var values = make([]int, 100)

func value(p point) int {
	i := coordIndex[p]
	if i >= len(values) {
		return 0
	}
	fmt.Println(i, len(values))
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
	buildIndex(n)
	values = make([]int, 80)
	values[1] = 1
	for i := 2; i <= n; i++ {
		p := coords[i]
		values[i] = summarize(p)
	}
	fmt.Println(values)
	return values[n]
}

func main() {
	fmt.Println(manhattan(1024))
	fmt.Println(manhattan(347991))
}
