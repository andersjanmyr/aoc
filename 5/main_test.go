package main

import (
	"testing"
)

type testCase struct {
	input    []int
	expected int
}

func TestValidPassphrase(t *testing.T) {
	cases := []testCase{
		{[]int{0, 3, 0, 1, -3}, 10},
	}
	for _, tc := range cases {
		r := exit(tc.input)
		if r != tc.expected {
			t.Errorf("Expected exit(%v) to be %d", tc.input, tc.expected)
		}
	}
}
