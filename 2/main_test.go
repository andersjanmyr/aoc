package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestChecksum(t *testing.T) {
	s := checksum([]int{5, 1, 9, 5})
	assert.Equal(t, 8, s)
	s = checksum([]int{7, 5, 3})
	assert.Equal(t, 4, s)
	s = checksum([]int{2, 4, 6, 8})
	assert.Equal(t, 6, s)
}

func TestDivisible(t *testing.T) {
	s := divisible([]int{5, 9, 2, 8})
	assert.Equal(t, 4, s)
	s = divisible([]int{9, 4, 3})
	assert.Equal(t, 3, s)
	s = divisible([]int{3, 8, 6, 5})
	assert.Equal(t, 2, s)
}

func TestTotal(t *testing.T) {
	matrix := [][]int{
		{5, 1, 9, 5},
		{7, 5, 3},
		{2, 4, 6, 8},
	}
	s := total(matrix)
	assert.Equal(t, 18, s)
}
