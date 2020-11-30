package main

import (
	"fmt"
	"reflect"
	"testing"
)

type testCase struct {
	input    []int
	expected []int
	i        []int
	o        []int
}

func TestIntCode(t *testing.T) {
	cases := []testCase{
		testCase{input: []int{1102, 34915192, 34915192, 7, 4, 7, 99, 0}, expected: []int{}, o: []int{1219070632396864}},
		testCase{input: []int{104, 1125899906842624, 99}, expected: []int{}, o: []int{1125899906842624}},

		testCase{input: []int{109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99}, o: []int{109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99}},
		// testCase{input: []int{1002, 4, 3, 4, 33}, expected: []int{1002, 4, 3, 4, 99}},
		// testCase{input: []int{1, 0, 0, 0, 99}, expected: []int{2, 0, 0, 0, 99}},
		// testCase{input: []int{2, 3, 0, 3, 99}, expected: []int{2, 3, 0, 6, 99}},
		// testCase{input: []int{2, 4, 4, 5, 99, 0}, expected: []int{2, 4, 4, 5, 99, 9801}},
		// testCase{input: []int{1, 1, 1, 4, 99, 5, 6, 0, 99}, expected: []int{30, 1, 1, 4, 2, 5, 6, 0, 99}},
		// testCase{input: []int{1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50},
		// 	expected: []int{3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50}},
		// testCase{input: []int{3, 0, 4, 0, 99}, expected: []int{1, 0, 4, 0, 99}, o: 1, i: []int{1}},
		// testCase{input: []int{3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8}, expected: []int{1, 0, 4, 0, 99}, i: []int{8}, o: 1},
		// testCase{input: []int{3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8}, expected: []int{1, 0, 4, 0, 99}, i: []int{5}, o: 0},
		// testCase{input: []int{3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8}, expected: []int{1, 0, 4, 0, 99}, i: []int{5}, o: 1},
		// testCase{input: []int{3, 3, 1108, -1, 8, 3, 4, 3, 99}, expected: []int{1, 0, 4, 0, 99}, i: []int{8}, o: 1},
		// testCase{input: []int{3, 3, 1108, -1, 8, 3, 4, 3, 99}, expected: []int{1, 0, 4, 0, 99}, i: []int{7}, o: 0},
		// testCase{input: []int{3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9}, expected: []int{1, 0, 4, 0, 99}, i: []int{7}, o: 1},
		// testCase{input: []int{3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9}, expected: []int{1, 0, 4, 0, 99}, i: []int{0}, o: 0},
		// testCase{input: []int{3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
		// 	1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
		// 	999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99}, expected: []int{}, i: []int{8}, o: 1000},
		// testCase{input: []int{3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
		// 	1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
		// 	999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99}, expected: []int{}, i: []int{5}, o: 999},
		// testCase{input: []int{3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
		// 	1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
		// 	999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99}, expected: []int{}, i: []int{22}, o: 1001},
	}

	for _, tc := range cases {
		o := make(chan int, 10)
		i := make(chan int, 10)
		var r []int
		go func() {
			defer close(o)
			defer close(i)
			r = intCode(tc.input, i, o)
		}()
		for _, n := range tc.i {
			go func(n int) {
				i <- n
			}(n)
		}
		fmt.Println(r)
		out := []int{}
		for i := range o {
			out = append(out, i)
		}
		if !reflect.DeepEqual(tc.o, out) {
			t.Errorf("Expected %#v, actual %#v", tc.o, out)
		}
	}
}

func TestMaxSignal(t *testing.T) {
	type testCase struct {
		input    []int
		expected int
	}
	cases := []testCase{
		testCase{input: []int{3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0}, expected: 43210},
		testCase{input: []int{3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0}, expected: 54321},
		testCase{input: []int{3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0}, expected: 65210},
	}
	for _, tc := range cases {
		r, seq := maxThrustSignal(tc.input)
		if r != tc.expected {
			t.Errorf("Expected %d, actual %d, seq: %d", tc.expected, r, seq)
		}
	}
}

func TestThrustSignal(t *testing.T) {
	type testCase struct {
		input    []int
		in       []int
		expected int
	}
	cases := []testCase{
		testCase{input: []int{3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0}, in: []int{4, 3, 2, 1, 0}, expected: 43210},
		testCase{input: []int{3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0}, in: []int{0, 1, 2, 3, 4}, expected: 54321},
		testCase{input: []int{3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0}, in: []int{1, 0, 4, 3, 2}, expected: 65210},
	}
	for _, tc := range cases {
		r := thrustSignal(tc.input, tc.in)
		if r != tc.expected {
			t.Errorf("Expected %d, actual %d", tc.expected, r)
		}
	}
}

func TestAmplifiedSignal(t *testing.T) {
	type testCase struct {
		input    []int
		in       []int
		expected int
	}
	cases := []testCase{
		testCase{input: []int{3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26,
			27, 4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5}, in: []int{9, 8, 7, 6, 5}, expected: 139629729},
		testCase{input: []int{3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
			-5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
			53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10}, in: []int{9, 7, 8, 5, 6}, expected: 18216},
	}
	for _, tc := range cases {
		r := amplifiedSignal(tc.input, tc.in)
		if r != tc.expected {
			t.Errorf("Expected %d, actual %d", tc.expected, r)
		}
	}
}

func TestMaxAmplifiedSignal(t *testing.T) {
	type testCase struct {
		input    []int
		expected int
	}
	cases := []testCase{
		testCase{input: []int{3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26,
			27, 4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5}, expected: 139629729},
		testCase{input: []int{3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
			-5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
			53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10}, expected: 18216},
	}
	for _, tc := range cases {
		r, seq := maxAmplifiedSignal(tc.input)
		if r != tc.expected {
			t.Errorf("Expected %d, actual %d, seq: %d", tc.expected, r, seq)
		}
	}
}
