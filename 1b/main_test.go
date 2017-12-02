package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSum(t *testing.T) {
	s := sum([]int{1, 2, 1, 2})
	assert.Equal(t, 6, s)
	s = sum([]int{1, 2, 2, 1})
	assert.Equal(t, 0, s)
	s = sum([]int{1, 2, 3, 4, 2, 5})
	assert.Equal(t, 4, s)
	s = sum([]int{1, 2, 1, 3, 1, 4, 1, 5})
	assert.Equal(t, 4, s)
}
