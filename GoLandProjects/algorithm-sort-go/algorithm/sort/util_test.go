package sort

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestLess(t *testing.T) {
	arr := []int{1, 2}
	assert.True(t, less(arr, 0, 1))
}

func TestExch(t *testing.T) {
	arr := []int{1, 2}
	exch(arr, 0, 1)
	assert.Equal(t, arr[0], 2)
	assert.Equal(t, arr[1], 1)
	assert.Equal(t, len(arr), 2)
}
