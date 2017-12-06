package main

import (
	"testing"
)

type testCase struct {
	input    []int
	expected int
}

func TestExit(t *testing.T) {
	cases := []testCase{
		{[]int{0, 2, 7, 0}, 4},
		{[]int{2, 8, 8, 5, 4, 2, 3, 1, 5, 5, 1, 2, 15, 13, 5, 14}, 1610},
	}
	for _, tc := range cases {
		r := reallocate(tc.input)
		if r != tc.expected {
			t.Errorf("Expected reallocate(%v) to be %d, actual %d", tc.input, tc.expected, r)
		}
	}
}
