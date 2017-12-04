package main

import (
	"testing"
)

type testCase struct {
	input    string
	expected bool
}

func TestValidPassphrase(t *testing.T) {
	cases := []testCase{
		{"aa bb cc dd ee", true},
		{"aa bb cc dd aa", false},
		{"aa bb cc aaa", true},
		{"abcde fghij ", true},
		{"abcde xyz ecdab ", false},
		{"iiii oiii ooii oooi oooo", true},
		{"oiii ioii iioi iiio", false},
	}
	for _, tc := range cases {
		r := validPassphrase(tc.input)
		if r != tc.expected {
			t.Errorf("Expected validPassphrase(%s) to be %b", tc.input, tc.expected)
		}
	}
}
