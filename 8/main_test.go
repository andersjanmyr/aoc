package main

import (
	"testing"
)

type testCase struct {
	input    string
	expected entry
}

func TestExit(t *testing.T) {
	cases := []testCase{
		{"b inc 5 if a > 1", entry{}},
		{"c inc -20 if c == 10", entry{}},
	}
	for _, tc := range cases {
		r := parseLine(tc.input)
		if r != tc.expected {
			t.Errorf("Expected parseLine(%v) to be %#v, actual %#v", tc.input, tc.expected, r)
		}
	}
}
