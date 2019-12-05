package main

import (
	"reflect"
	"testing"
)

type testCase struct {
	input    []int
	expected []int
}

func TestIntCode(t *testing.T) {
	cases := []testCase{
		testCase{input: []int{1, 0, 0, 0, 99}, expected: []int{2, 0, 0, 0, 99}},
		testCase{input: []int{2, 3, 0, 3, 99}, expected: []int{2, 3, 0, 6, 99}},
		testCase{input: []int{2, 4, 4, 5, 99, 0}, expected: []int{2, 4, 4, 5, 99, 9801}},
		testCase{input: []int{1, 1, 1, 4, 99, 5, 6, 0, 99}, expected: []int{30, 1, 1, 4, 2, 5, 6, 0, 99}},
		testCase{input: []int{1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50},
			expected: []int{3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50}},
	}
	for _, tc := range cases {
		r := intCode(tc.input)
		if !reflect.DeepEqual(r, tc.expected) {
			t.Errorf("Expected %d, actual %d", tc.expected, r)
		}
	}
}
