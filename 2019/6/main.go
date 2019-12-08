package main

import (
	"fmt"
	"strings"
)

func main() {
	input := lines()
	fmt.Println(input)
	fmt.Println(dijkstra(input))
}

var edges = map[string][]string{}

func makeGraph(lines []string) {
	for _, l := range lines {
		ss := strings.Split(l, ")")
		addEdge(ss[0], ss[1])
	}
}

func orbits(lines []string) int {
	makeGraph(lines)
	r := root()
	fmt.Println(edges, r)
	return traverse(r, 0)
}

func dijkstra(lines []string) int {
	makeGraph(lines)
	r := root()
	fmt.Println(edges, r)
	_, b := bfs(r, "SAN", 0)
	_, d := bfs(r, "YOU", 0)

	e := reverse(b)
	f := reverse(d)
	fmt.Println(e, f)
	i := 0
	for {
		if e[i] != f[i] {
			fmt.Println(e[i:])
			return len(e[i:]) + len(f[i:]) - 2
		}
		i++
	}
	return 2
}

func reverse(input []string) []string {
	if len(input) == 0 {
		return input
	}
	return append(reverse(input[1:]), input[0])
}
func bfs(from, to string, depth int) (int, []string) {
	if from == to {
		return depth, []string{to}
	}
	nodes := edges[from]
	if len(nodes) == 0 {
		return 1234567, nil
	}
	fmt.Println(nodes)
	ds := make([]int, len(nodes))
	paths := make([][]string, len(nodes))
	for i, n := range nodes {
		ds[i], paths[i] = bfs(n, to, depth+1)
	}
	d, path := min(ds, paths)
	return d, append(path, from)
}

func min(is []int, paths [][]string) (int, []string) {
	var m, index int
	for i, e := range is {
		if i == 0 || e < m {
			m = e
			index = i
		}
	}
	return is[index], paths[index]
}

func traverse(root string, depth int) int {
	nodes := edges[root]
	c := depth
	for _, n := range nodes {
		c += traverse(n, depth+1)
	}
	return c
}

func addEdge(a, b string) {
	es := edges[a]
	if len(es) > 0 {
		edges[a] = append(es, b)
	} else {
		edges[a] = []string{b}
	}
}

func root() string {
	sum := 0
	m := map[string]int{}
	for k, v := range edges {
		m[k] += len(v)
		sum += len(v)
	}
	for _, vs := range edges {
		for _, v := range vs {
			delete(m, v)
		}
	}
	for k, _ := range m {
		return k
	}
	return ""
}
