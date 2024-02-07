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
	"strings"
)

func separateLines(inp [][]string, gapSize int8) string {
	// dont ask this is literally black magic with 0 optimisation but who cares its only for the cli
	var output [][]string
	for i := 0; i < len(inp); i++ {
		output = append(output, []string{})
		for j := 0; j < len(inp[i]); j++ {
			output[i] = append(output[i], "")
		}
	}

	if len(inp) == 0 {
		return ""
	}

	// Make sure that every one of the inner arrays is the same size.
	longestLine := 0
	for i := 0; i < len(inp); i++ {
		if len(inp[i]) > longestLine {
			longestLine = len(inp[i])
		}
	}

	for i := 0; i < len(inp); i++ {
		for j := 0; j < len(inp[i])-longestLine; j++ {
			inp[i] = append(inp[i], "")
		}
	}

	for i := 0; i < len(output); i++ {
		for j := 0; j < len(output[i])-longestLine; j++ {
			output[i] = append(output[i], "")
		}
	}

	// For each column in the matrix
	for i := 0; i < len(inp[0]); i++ {
		longestInCol := 0
		for j := 0; j < len(inp); j++ {
			if len(inp[j][i]) > longestInCol {
				longestInCol = len(inp[j][i])
			}
		}
		charsBeforeNextCol := longestInCol + int(gapSize)

		// For each row (to access all elements over this iteration)
		for j := 0; j < len(inp); j++ {
			item := inp[j][i]
			output[j][i] = inp[j][i]
			for n := 0; n < charsBeforeNextCol-len(item); n++ {
				output[j][i] += " "
			}
		}
	}

	lines := []string{}
	for i := 0; i < len(output); i++ {
		lines = append(lines, stripWhitespace(strings.Join(output[i], "")))
	}
	return strings.Join(lines, "\n")
}

func leftPad(inp string, paddingSize int) string {
	output := ""
	for i := 0; i < paddingSize; i++ {
		output += " "
	}
	output += inp
	return output
}

func stripWhitespace(inp string) string {
	output := inp
	for i := len(inp) - 1; i > 0; i-- {
		if inp[i] == ' ' {
			output = output[0 : len(output)-1]
		} else {
			return output
		}
	}
	return output
}

func merge(a []int, b []int) []int {
	final := []int{}
	i := 0
	j := 0
	for i < len(a) && j < len(b) {
		if a[i] < b[j] {
			final = append(final, a[i])
			i++
		} else {
			final = append(final, b[j])
			j++
		}
	}
	for ; i < len(a); i++ {
		final = append(final, a[i])
	}
	for ; j < len(b); j++ {
		final = append(final, b[j])
	}
	return final
}

func mergeSort(items []int) []int {
	if len(items) < 2 {
		return items
	}
	first := mergeSort(items[:len(items)/2])
	second := mergeSort(items[len(items)/2:])
	return merge(first, second)
}

func reverseArr(inp []int) []int {
	for i := 0; i < len(inp)/2; i++ {
		tmp := inp[i]
		inp[i] = inp[len(inp)-(i+1)]
		inp[len(inp)-(i+1)] = tmp
	}
	return inp
}

func backwardsSort(inp []int) []int {
	return reverseArr(mergeSort(inp))
}

func remove(slice []string, i int) []string {
	return append(slice[:i], slice[i+1:]...)
}
