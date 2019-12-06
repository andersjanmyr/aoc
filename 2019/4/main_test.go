package main

import (
	"testing"
)

type testCase struct {
	input    int
	expected bool
}

func TestFindCrossPoint(t *testing.T) {
	cases := []testCase{
		testCase{input: 111111, expected: false},
		testCase{input: 223450, expected: false},
		testCase{input: 123789, expected: false},
		testCase{input: 112233, expected: true},
		testCase{input: 123444, expected: false},
		testCase{input: 111122, expected: true},
	}
	for _, tc := range cases {
		r := isValid(tc.input)
		if r != tc.expected {
			t.Errorf("Input: %d, expected %t, actual %t", tc.input, tc.expected, r)
		}
	}
}
