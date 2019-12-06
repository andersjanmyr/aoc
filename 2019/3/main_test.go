package main

import (
	"testing"
)

type testCase struct {
	input    [][]string
	expected int
}

func TestFindCrossPoint(t *testing.T) {
	cases := []testCase{
		testCase{input: [][]string{
			{"R8", "U5", "L5", "D3"},
			{"U7", "R6", "D4", "L4"}}, expected: 30},
		testCase{input: [][]string{
			{"R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"},
			{"U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"}}, expected: 610},
		testCase{input: [][]string{
			{"R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51"},
			{"U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7"}}, expected: 410},
	}
	for _, tc := range cases {
		r := findCrossPoint(tc.input[0], tc.input[1])
		if r != tc.expected {
			t.Errorf("Expected %d, actual %d", tc.expected, r)
		}
	}
}
