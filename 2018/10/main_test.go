package main

import (
	"reflect"
	"testing"

	"github.com/stretchr/testify/assert"
)

type testCase struct {
	n        int
	lengths  []int
	expected int
}

func TestHash(t *testing.T) {
	cases := []testCase{
		{5, []int{3, 4, 1, 5}, 12},
		// {256, []int{199, 0, 255, 136, 174, 254, 227, 16, 51, 85, 1, 2, 22, 17, 7, 192}, 12},
	}
	for _, tc := range cases {
		r := hash(tc.n, tc.lengths)
		if r != tc.expected {
			t.Errorf("Expected hash(%v) to be %#v, actual %#v", tc.lengths, tc.expected, r)
		}
	}
}

type testCase2 struct {
	input    []int
	pos      int
	length   int
	expected []int
}

func TestReverse(t *testing.T) {
	cases := []testCase2{
		{[]int{0, 1, 2, 3, 4}, 0, 3, []int{2, 1, 0, 3, 4}},
		{[]int{2, 1, 0, 3, 4}, 3, 4, []int{4, 3, 0, 1, 2}},
		{[]int{4, 3, 0, 1, 2}, 3, 1, []int{4, 3, 0, 1, 2}},
		{[]int{4, 3, 0, 1, 2}, 1, 5, []int{3, 4, 2, 1, 0}},
	}
	for _, tc := range cases {
		r := reverse(tc.input, tc.pos, tc.length)
		if !reflect.DeepEqual(r, tc.expected) {
			t.Errorf("Expected reverse(%v) to be %#v, actual %#v", tc.input, tc.expected, r)
		}
	}
}

func TestDenseHashItem(t *testing.T) {
	ns := []int{65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22}
	assert.Equal(t, 64, denseHashItem(ns))
	ns = []int{1, 2, 4, 8, 16, 32, 64, 128}
	assert.Equal(t, 255, denseHashItem(ns))
}

type testCase3 struct {
	input    []int
	expected string
}

func TestDenseHash(t *testing.T) {
	cases := []testCase3{
		{nums(256), "0f1f2f3f4f5f6f7f8f9fafbfcfdfefff"},
		{make([]int, 256), "00000000000000000000000000000000"},
	}
	for _, tc := range cases {
		r := denseHash(tc.input)
		if r != tc.expected {
			t.Errorf("Expected denseHash(%v) to be %#v, actual %#v", tc.input, tc.expected, r)
		}
	}
}

type testCase4 struct {
	input    string
	expected string
}

func TestSuperHash(t *testing.T) {
	cases := []testCase4{
		{"", "a2582a3a0e66e6e86e3812dcb672a272"},
		{"AoC 2017", "33efeb34ea91902bb2f59c9920caa6cd"},
		{"1,2,3", "3efbe78a8d82f29979031a4aa0b16a9d"},
		{"1,2,4", "63960835bcdc130f0b66d7ff4f6a5a8e"},
	}
	for _, tc := range cases {
		r := knotHash(tc.input)
		if r != tc.expected {
			t.Errorf("Expected knotHash(%v) to be %#v, actual %#v", tc.input, tc.expected, r)
		}
	}
}
