package argparse

import "strings"

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
		for j := 0; j < len(output[i]); j++ {
			output[i] = append(output[i], "")
		}
	}

	for i := 0; i < len(inp[0]); i++ {
		longestInCol := 0
		for j := 0; j < len(inp); j++ {
			if len(inp[j][i]) > longestInCol {
				longestInCol = len(inp[j][i])
			}
		}

		charsBeforeNextCol := longestInCol + int(gapSize)

		for j := 0; j < len(inp); j++ {
			lines := inp[j]
			item := lines[i]
			output[j][i] = inp[j][i]
			for n := 0; n < charsBeforeNextCol-len(item); n++ {
				output[j][i] += ""
			}
		}
	}

	lines := []string{}
	for i := 0; i < len(output); i++ {
		lines = append(lines, strings.Join(output[i], ""))
	}
	return strings.Join(lines, "\n")
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