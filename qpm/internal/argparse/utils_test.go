//  QQML or the Quiz Question Markup Language.
//  Copyright (C) 2023 'mppbtw'
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program. If not, see <https://www.gnu.org/licenses/>.

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

func TestMergeSort(T *testing.T) {
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

func TestSeparateLines(T *testing.T) {
	input := [][]string{{"a", "b"}, {"cc", "dd"}}
	expected := "a   b\ncc  dd"
	result := separateLines(input, 2)
	if result != expected {
		T.Error("Expected:\n" + expected + "\nResult:\n" + result)
	}
}

func TestStripWhitespace(T *testing.T) {
	input := " 1 2 3   "
	expected := " 1 2 3"
	result := stripWhitespace(input)
	if result != expected {
		T.Error("Expected:\n" + expected + "\nResult:\n" + result)
	}
}

func TestLeftPad(T *testing.T) {
	input := "belief doesnt change reality gary"
	expected := "    belief doesnt change reality gary"
	result := leftPad(input, 4)
	if result != expected {
		T.Error("Expected:\n" + expected + "\nResult:\n" + result)
	}
}
t)
	}
}
