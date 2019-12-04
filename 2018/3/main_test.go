package main

import (
	"testing"
)

/*
37 36  35  34  33  32 31
38 17  16  15  14  13 30
39 18   5   4   3  12 29
40 19   6   1   2  11 28
41 20   7   8   9  10 27
42 21  22  23  24  25 26 51
43 44  45  46  47  48 49 50
*/
func TestManhattanChecksum(t *testing.T) {
	cases := [][]int{
		{0, 1},
		{1, 2},
		{2, 3},
		{1, 4},
		{2, 5},
		{1, 6},
		{2, 7},
		{1, 8},
		{2, 9},
		{3, 10},
		{2, 11},
		{4, 13},
		{4, 21},
		{2, 23},
		{3, 24},
		{4, 25},
		{5, 26},
		{3, 28},
		{5, 30},
		{6, 49},
		{6, 51},
		{31, 1024},
		{480, 347991},
	}
	for _, c := range cases {
		r := manhattan(c[1])
		if r != c[0] {
			t.Errorf("Expected manhattan(%d) to equal: %d, actual %d", c[1], c[0], r)
		}
	}
}

/*
147  142  133  122   59
304    5    4    2   57
330   10    1    1   54
351   11   23   25   26
362  747  806--->   ...
*/
func TestSquareSum(t *testing.T) {
	cases := [][]int{
		{1, 1},
		{1, 2},
		{2, 3},
		{4, 4},
		{5, 5},
		{10, 6},
		{11, 7},
		{23, 8},
		{25, 9},
		{26, 10},
		{54, 11},
		{59, 13},
		{806, 23},
		{349975, 63},
	}
	for _, c := range cases {
		r := squareSum(c[1])
		if r != c[0] {
			t.Errorf("Expected squareSum(%d) to equal: %d, actual %d", c[1], c[0], r)
		}
	}
}
