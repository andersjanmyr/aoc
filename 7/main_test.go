package main

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

type testCase struct {
	input    []string
	expected string
}

func TestExit(t *testing.T) {
	cases := []testCase{
		testCase{
			input: []string{
				"pbga (66)",
				"xhth (57)",
				"ebii (61)",
				"havc (66)",
				"ktlj (57)",
				"fwft (72) -> ktlj, cntj, xhth",
				"qoyq (66)",
				"padx (45) -> pbga, havc, qoyq",
				"tknk (41) -> ugml, padx, fwft",
				"jptl (61)",
				"ugml (68) -> gyxo, ebii, jptl",
				"gyxo (61)",
				"cntj (57)",
			},
			expected: "tknk",
		},
	}
	for _, tc := range cases {
		r := findRoot(tc.input)
		if r != tc.expected {
			t.Errorf("Expected %s, actual %s", tc.expected, r)
		}
	}
}

func TestCalcSubtrees(t *testing.T) {
	cases := []testCase{
		testCase{
			input: []string{
				"pbga (66)",
				"xhth (57)",
				"ebii (61)",
				"havc (66)",
				"ktlj (57)",
				"fwft (72) -> ktlj, cntj, xhth",
				"qoyq (66)",
				"padx (45) -> pbga, havc, qoyq",
				"tknk (41) -> ugml, padx, fwft",
				"jptl (61)",
				"ugml (68) -> gyxo, ebii, jptl",
				"gyxo (61)",
				"cntj (57)",
			},
			expected: "tknk",
		},
	}
	for _, tc := range cases {
		calcSubtrees(tc.input, "tknk")
		// if r != tc.expected {
		// 	t.Errorf("Expected %s, actual %s", tc.expected, r)
		// }
	}
}

func TestParseLine(t *testing.T) {
	expected := entry{
		key:    "ugml",
		value:  68,
		leaves: []string{"gyxo", "ebii", "jptl"},
	}
	actual := parseLine("ugml (68) -> gyxo, ebii, jptl")
	assert.Equal(t, actual, expected)
}

func TestParseLine2(t *testing.T) {
	expected := entry{
		key:    "ugml",
		value:  68,
		leaves: []string{},
	}
	actual := parseLine("ugml (68)")
	fmt.Println(actual)
	assert.Equal(t, actual, expected)
}
