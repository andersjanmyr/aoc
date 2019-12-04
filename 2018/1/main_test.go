package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSum(t *testing.T) {
	s := sum([]int{1, 1, 2, 2})
	assert.Equal(t, 3, s)
	s = sum([]int{1, 1, 1, 1})
	assert.Equal(t, 4, s)
	s = sum([]int{1, 2, 3, 4})
	assert.Equal(t, 0, s)
	s = sum([]int{9, 1, 2, 1, 2, 1, 2, 9})
	assert.Equal(t, 9, s)
}
