package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func main() {
	input := lines()
	root := calcSubtrees(input, "cqmvs")
	fmt.Println(root)
}

type entry struct {
	key    string
	value  int
	leaves []string
}

type Entries []entry

func (e Entries) Len() int {
	return len(e)
}
func (e Entries) Swap(i, j int) {
	e[i], e[j] = e[j], e[i]
}
func (e Entries) Less(i, j int) bool {
	if len(e[j].leaves) == 0 {
		return false
	}
	for _, v := range e[i].leaves {
		if v == e[j].key {
			return false
		}
	}
	return true
}

func calcSubtrees(input []string, root string) int {
	entries := getEntries(input)
	entryMap := make(map[string]entry)
	for _, e := range entries {
		entryMap[e.key] = e
	}
	v := calcSubtree(root, entryMap)
	return v
}

func calcSubtree(root string, entryMap map[string]entry) int {
	e := entryMap[root]
	if len(e.leaves) == 0 {
		return e.value
	}
	n := e.value
	tvs := make([]int, len(e.leaves))
	for i, l := range e.leaves {
		tvs[i] = calcSubtree(l, entryMap)
		n += tvs[i]
	}
	if !allEqual(tvs) {
		fmt.Printf("%#v %#v\n", e, tvs)
		for _, l := range e.leaves {
			fmt.Printf("%#v\n", entryMap[l])
		}
		os.Exit(0)
	}
	return n
}

func allEqual(tvs []int) bool {
	if len(tvs) == 0 {
		return true
	}
	n := tvs[0]
	for i := 1; i < len(tvs); i++ {
		if n != tvs[i] {
			return false
		}
	}
	return true
}

func getEntries(input []string) []entry {
	entries := make([]entry, len(input))
	for i, v := range input {
		entries[i] = parseLine(v)
	}
	return entries
}

func findRoot(input []string) string {
	entries := getEntries(input)
	insertionSort(entries)
	return entries[len(entries)-1].key
}

func insertionSort(entries Entries) {
	for i := 1; i < len(entries); i++ {
		for j := i; j > 0 && entries.Less(j, j-1); j-- {
			entries.Swap(j, j-1)
		}
	}
}

// ugml (68) -> gyxo, ebii, jptl
func parseLine(line string) entry {
	re := regexp.MustCompile("([a-z]*) \\((\\d*)\\)( -> (.*))?")
	md := re.FindStringSubmatch(line)
	// fmt.Printf("%s\n%v\n", line, md[1:])
	value, _ := strconv.Atoi(md[2])
	var leaves []string
	if len(md[4]) > 0 {
		leaves = strings.Split(md[4], ", ")
	} else {
		leaves = []string{}
	}
	return entry{
		key:    md[1],
		value:  value,
		leaves: leaves,
	}
}
