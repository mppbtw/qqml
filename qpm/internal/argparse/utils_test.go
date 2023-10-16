package argparse

import (
	"reflect"
	"testing"
)

type TestArrayTransformData struct {
	input    []int
	expected []int
}

func TestReverse(T *testing.T) {
	data := []TestArrayTransformData{
		{
			input:    []int{1, 2, 3, 4, 5},
			expected: []int{5, 4, 3, 2, 1},
		},
		{
			input:    []int{1, 2, 3, 4, 5},
			expected: []int{5, 4, 3, 2, 1},
		},
		{
			input:    []int{2, 3, 1, 18, 4},
			expected: []int{4, 18, 1, 3, 2},
		},
		{
			input:    []int{2, 15, 198, 82},
			expected: []int{82, 198, 15, 2},
		},
	}
	for _, datum := range data {
		result := reverseArr(datum.input)
		if !reflect.DeepEqual(result, datum.expected) {
			T.Error("Expected:", datum.expected, "Result:", result)
		}
	}
}

func TestMerge(T *testing.T) {
	data := []TestArrayTransformData{
		{
			input:    []int{5, 4, 3, 2, 1},
			expected: []int{1, 2, 3, 4, 5},
		},
		{
			input:    []int{23, 12, -2, 2},
			expected: []int{-2, 2, 12, 23},
		},
		{
			input:    []int{2, 3, 2, 1, 0},
			expected: []int{0, 1, 2, 2, 3},
		},
		{
			input:    []int{2, 3, 2, 1, 0},
			expected: []int{0, 1, 2, 2, 3},
		},
	}
	for _, datum := range data {
		result := mergeSort(datum.input)
		if !reflect.DeepEqual(result, datum.expected) {
			T.Error("Expected:", datum.expected, "Result:", result)
		}
	}
}

func TestBackwardsSort(T *testing.T) {
	data := []TestArrayTransformData{
		{
			expected: []int{5, 4, 3, 2, 1},
			input:    []int{1, 2, 3, 4, 5},
		},
		{
			input:    []int{23, 12, -2, 2},
			expected: []int{23, 12, 2, -2},
		},
		{
			input:    []int{2, 3, 2, 1, 0},
			expected: []int{3, 2, 2, 1, 0},
		},
		{
			input:    []int{0, 8, -1, -10},
			expected: []int{8, 0, -1, -10},
		},
	}

	for _, datum := range data {
		result := backwardsSort(datum.input)
		if !reflect.DeepEqual(result, datum.expected) {
			T.Error("Expected:", datum.expected, "Result:", result)
		}
	}
}
