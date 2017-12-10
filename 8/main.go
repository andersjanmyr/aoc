package main

import (
	"fmt"
	"regexp"
	"strconv"
)

var regs map[string]int
var maxa int
var rega string

func main() {
	regs = make(map[string]int)
	input := lines()
	// input := []string{
	// 	"b inc 5 if a > 1",
	// 	"a inc 1 if b < 5",
	// 	"c dec -10 if a >= 1",
	// 	"c inc -20 if c == 10",
	// }
	entries := parseLines(input)
	evalEntries(entries)
	var max int
	var reg string
	for k, v := range regs {
		if v > max {
			max = v
			reg = k
		}
	}
	fmt.Println(reg, max)
	fmt.Println(rega, maxa)
}

type entry struct {
	regi         string
	action       string
	actionAmount int
	rego         string
	cond         string
	condAmount   int
}

func parseLines(lines []string) []entry {
	entries := make([]entry, len(lines))
	for i := 0; i < len(lines); i++ {
		entries[i] = parseLine(lines[i])
	}
	return entries
}

// j inc -19 if jhb >= 10
func parseLine(line string) entry {
	re := regexp.MustCompile("([a-z]*) ([a-z]*) ([-0-9]*) if ([a-z]*) ([><!=]*) ([-0-9]*)")
	md := re.FindStringSubmatch(line)
	fmt.Printf("%s\n%v\n", line, md[1:])
	actionAmount, _ := strconv.Atoi(md[3])
	condAmount, _ := strconv.Atoi(md[6])
	return entry{
		regi:         md[1],
		action:       md[2],
		actionAmount: actionAmount,
		rego:         md[4],
		cond:         md[5],
		condAmount:   condAmount,
	}
}

func evalEntries(entries []entry) {
	for i := 0; i < len(entries); i++ {
		evalEntry(entries[i])
	}
}

func evalEntry(e entry) {
	if compare(read(e.rego), e.condAmount, e.cond) {
		amount := calc(read(e.regi), e.actionAmount, e.action)
		write(e.regi, amount)
		if amount > maxa {
			maxa = amount
			rega = e.regi
		}
	}
}

func compare(a1, a2 int, cond string) bool {
	switch cond {
	case "==":
		return a1 == a2
	case "!=":
		return a1 != a2
	case ">":
		return a1 > a2
	case "<":
		return a1 < a2
	case ">=":
		return a1 >= a2
	case "<=":
		return a1 <= a2
	default:
		panic(cond)
	}
}

func calc(a1, a2 int, action string) int {
	switch action {
	case "inc":
		return a1 + a2
	case "dec":
		return a1 - a2
	default:
		panic(action)
	}
}

func read(reg string) int {
	n, ok := regs[reg]
	if !ok {
		return 0
	}
	return n
}

func write(reg string, n int) {
	regs[reg] = n
}
