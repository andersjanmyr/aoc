package main

import (
	"testing"
)

type testCase struct {
	input    []int
	expected []int
	i        int
	o        int
}

func TestIntCode(t *testing.T) {
	cases := []testCase{
		testCase{input: []int{1002, 4, 3, 4, 33}, expected: []int{1002, 4, 3, 4, 99}},
		testCase{input: []int{1, 0, 0, 0, 99}, expected: []int{2, 0, 0, 0, 99}},
		testCase{input: []int{2, 3, 0, 3, 99}, expected: []int{2, 3, 0, 6, 99}},
		testCase{input: []int{2, 4, 4, 5, 99, 0}, expected: []int{2, 4, 4, 5, 99, 9801}},
		testCase{input: []int{1, 1, 1, 4, 99, 5, 6, 0, 99}, expected: []int{30, 1, 1, 4, 2, 5, 6, 0, 99}},
		testCase{input: []int{1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50},
			expected: []int{3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50}},
		testCase{input: []int{3, 0, 4, 0, 99}, expected: []int{1, 0, 4, 0, 99}, o: 1, i: 1},
		testCase{input: []int{3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8}, expected: []int{1, 0, 4, 0, 99}, i: 8, o: 1},
		testCase{input: []int{3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8}, expected: []int{1, 0, 4, 0, 99}, i: 5, o: 0},
		testCase{input: []int{3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8}, expected: []int{1, 0, 4, 0, 99}, i: 5, o: 1},
		testCase{input: []int{3, 3, 1108, -1, 8, 3, 4, 3, 99}, expected: []int{1, 0, 4, 0, 99}, i: 8, o: 1},
		testCase{input: []int{3, 3, 1108, -1, 8, 3, 4, 3, 99}, expected: []int{1, 0, 4, 0, 99}, i: 7, o: 0},
		testCase{input: []int{3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9}, expected: []int{1, 0, 4, 0, 99}, i: 7, o: 1},
		testCase{input: []int{3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9}, expected: []int{1, 0, 4, 0, 99}, i: 0, o: 0},
		testCase{input: []int{3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
			1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
			999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99}, expected: []int{}, i: 8, o: 1000},
		testCase{input: []int{3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
			1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
			999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99}, expected: []int{}, i: 5, o: 999},
		testCase{input: []int{3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
			1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
			999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99}, expected: []int{}, i: 22, o: 1001},
	}

	for _, tc := range cases {
		_, out := intCode(tc.input, tc.i)
		if out != tc.o {
			t.Errorf("Expected %d, actual %d", tc.o, out)
		}
		// if !reflect.DeepEqual(r, tc.expected) {
		// 	t.Errorf("Expected %d, actual %d", tc.expected, r)
		// }
	}
}
