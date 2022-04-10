package sort

import "golang.org/x/exp/constraints"

func less[T constraints.Ordered](arr []T, lo int, hi int) bool {
	return arr[lo] < arr[hi]
}

func exch[T any](arr []T, lo int, hi int) {
	tmp := arr[lo]
	arr[lo] = arr[hi]
	arr[hi] = tmp
}
