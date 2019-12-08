package main

import (
	"testing"
)

type testCase struct {
	input    []string
	expected int
}

func TestFindCrossPoint(t *testing.T) {
	cases := []testCase{
		testCase{input: []string{
			"COM)B",
			"B)C",
			"C)D",
			"D)E",
			"E)F",
			"B)G",
			"G)H",
			"D)I",
			"E)J",
			"J)K",
			"K)L",
			"K)YOU",
			"I)SAN",
		}, expected: 4},
	}
	for _, tc := range cases {
		r := dijkstra(tc.input)
		if r != tc.expected {
			t.Errorf("Expected %d, actual %d", tc.expected, r)
		}
	}
}
