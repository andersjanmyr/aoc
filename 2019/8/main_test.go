package main

import (
	"reflect"
	"testing"
)

func TestLayers(t *testing.T) {
	type testCase struct {
		input    string
		w, h     int
		expected [][]string
	}

	cases := []testCase{
		testCase{input: "123456789012", w: 3, h: 2,
			expected: [][]string{
				{"123", "456"},
				{"789", "012"},
			}},
	}
	for _, tc := range cases {
		r := layers(tc.input, tc.w, tc.h)
		if !reflect.DeepEqual(r, tc.expected) {
			t.Errorf("Expected %#v, actual %#v", tc.expected, r)
		}
	}
}

func TestMinLayers(t *testing.T) {
	type testCase struct {
		input    string
		w, h     int
		expected []string
	}

	cases := []testCase{
		testCase{input: "123456789012", w: 3, h: 2,
			expected: []string{"123", "456"},
		},
	}
	for _, tc := range cases {
		r := minLayer(tc.input, tc.w, tc.h)
		if !reflect.DeepEqual(r, tc.expected) {
			t.Errorf("Expected %#v, actual %#v", tc.expected, r)
		}
	}
}

func TestDecode(t *testing.T) {
	type testCase struct {
		input    string
		w, h     int
		expected []string
	}

	cases := []testCase{
		testCase{input: "0222112222120000", w: 2, h: 2,
			expected: []string{"01", "10"},
		},
	}
	for _, tc := range cases {
		r := decode(tc.input, tc.w, tc.h)
		if !reflect.DeepEqual(r, tc.expected) {
			t.Errorf("Expected %#v, actual %#v", tc.expected, r)
		}
	}
}
