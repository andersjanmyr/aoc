package main

import (
	"testing"
)

type testCase struct {
	input    int
	expected int
}

func TestRequiredFuel1(t *testing.T) {
	cases := []testCase{
		testCase{input: 12, expected: 2},
		testCase{input: 14, expected: 2},
		testCase{input: 1969, expected: 654},
		testCase{input: 100756, expected: 33583},
	}
	for _, tc := range cases {
		r := requiredFuel1(tc.input)
		if r != tc.expected {
			t.Errorf("Expected %d, actual %d", tc.expected, r)
		}
	}
}

func TestRequiredFuel(t *testing.T) {
	cases := []testCase{
		testCase{input: 1969, expected: 966},
		testCase{input: 100756, expected: 50346},
	}
	for _, tc := range cases {
		r := requiredFuel(tc.input)
		if r != tc.expected {
			t.Errorf("Expected %d, actual %d", tc.expected, r)
		}
	}
}
