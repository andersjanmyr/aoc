package main

import (
	"fmt"
	"strconv"
)

type coord struct {
	x, y int
}

var o coord = coord{0, 0}

func (c coord) add(a coord) coord {
	return coord{c.x + a.x, c.y + a.y}
}

var moveToDelta = map[string]coord{}

func init() {
	moveToDelta["R"] = coord{1, 0}
	moveToDelta["L"] = coord{-1, 0}
	moveToDelta["U"] = coord{0, 1}
	moveToDelta["D"] = coord{0, -1}
}

func main() {
	input := toStringMatrix(lines())
	fmt.Println(input)
	fmt.Println(findCrossPoint(input[0], input[1]))
}

func findCrossPoint(a, b []string) int {
	as := getCoordinates(o, a)
	bs := getCoordinates(o, b)
	cs := findMatches(as, bs)
	ias := steps(as, cs)
	ibs := steps(bs, cs)
	fmt.Println(ias, ibs)
	is := []int{}
	for i, a := range ias {
		is = append(is, a+ibs[i])
	}
	return min(is)
}

func getCoordinates(c coord, moves []string) []coord {
	coords := []coord{}
	for _, m := range moves {
		cs := getCoordinates1(c, m)
		coords = append(coords, cs...)
		c = cs[len(cs)-1]
	}
	return coords
}

func getCoordinates1(c coord, move string) []coord {
	dir, dist := getMove(move)
	coords := []coord{}
	delta := moveToDelta[dir]
	for i := 0; i < dist; i++ {
		c = c.add(delta)
		coords = append(coords, c)
	}
	return coords
}

func getMove(move string) (string, int) {
	dir := move[0:1]
	dist, _ := strconv.Atoi(move[1:])
	return dir, dist
}

func findMatches(as, bs []coord) []coord {
	cs := []coord{}
	for _, a := range as {
		for _, b := range bs {
			if a == b {
				cs = append(cs, a)
			}
		}
	}
	return cs
}

func manhattanDistances(cs []coord) []int {
	is := []int{}
	for _, c := range cs {
		is = append(is, abs(c.x)+abs(c.y))
	}
	return is
}

func abs(i int) int {
	if i < 0 {
		return -i
	}
	return i
}

func min(is []int) int {
	var m int
	for i, e := range is {
		if i == 0 || e < m {
			m = e
		}
	}
	return m
}

func steps(as, cs []coord) []int {
	is := []int{}
	for _, c := range cs {
		for i, a := range as {
			if a == c {
				is = append(is, i+1)
			}
		}
	}
	return is
}
