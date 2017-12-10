package main

import (
	"testing"
)

type testCase struct {
	input    string
	expected int
}

func TestCalcGroups(t *testing.T) {
	cases := []testCase{
		{"{}", 1},
		{"{{{}}}", 6},
		{"{{},{}}", 5},
		{"{{{},{},{{}}}}", 16},
		{"{<a>,<a>,<a>,<a>}", 1},
		{"{{<ab>},{<ab>},{<ab>},{<ab>}}", 9},
		{"{{<!!>},{<!!>},{<!!>},{<!!>}}", 9},
		{"{{<a!>},{<a!>},{<a!>},{<ab>}}", 3},
	}
	for _, tc := range cases {
		r, _ := calcGroups([]byte(tc.input))
		if r != tc.expected {
			t.Errorf("Expected parseLine(%v) to be %#v, actual %#v", tc.input, tc.expected, r)
		}
	}
}

func TestSkipGarbage(t *testing.T) {
	cases := []testCase{
		{"<>,", 0},
		{"<random characters>", 17},
		{"<<<<>", 3},
		{"<{!>}>", 2},
		{"<!!>", 0},
		{"<!!!>>", 0},
		{"<{o\"i!a,<{i<a>", 10},
	}
	for _, tc := range cases {
		_, gc := skipGarbage([]byte(tc.input), 0)
		if gc != tc.expected {
			t.Errorf("Expected skipGarbage(%s) to return %d, actual %d", tc.input, tc.expected, gc)
		}
	}
}
