package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	var inputLines []string

	for scanner.Scan() {
		line := scanner.Text()
		inputLines = append(inputLines, line)
	}

	if err := scanner.Err(); err != nil {
		fmt.Println("Error reading input:", err)
		return
	}

	result1 := part1(inputLines)
	result2 := part2(inputLines)
	fmt.Println("Result1:", result1)
	fmt.Println("Result2:", result2)
}

func str_to_int(input []string) []int {
	var output []int

	for i := 0; i < len(input); i++ {
		val, _ := strconv.Atoi(input[i])
		output = append(output, val)
	}

	return output
}

func safe_increasing(reports []int) bool {
	for i := 1; i < len(reports); i++ {
		diff := reports[i] - reports[i-1]
		if diff <= 0 || diff > 3 {
			return false
		}
	}
	return true
}

func safe_decreasing(reports []int) bool {
	for i := 1; i < len(reports); i++ {
		diff := reports[i-1] - reports[i]
		if diff <= 0 || diff > 3 {
			return false
		}
	}
	return true
}

func part1(input []string) int {

	safe := 0
	for _, line := range input {
		levels := str_to_int(strings.Fields(line))

		if safe_decreasing(levels) || safe_increasing(levels) {
			safe++
		}

	}

	return safe
}

func part2(input []string) int {
	safe := 0
	for _, line := range input {
		levels := str_to_int(strings.Fields(line))

		if safe_decreasing(levels) || safe_increasing(levels) {
			safe++
			continue
		}
		pointer := 0
		found_safe := false

		for pointer < len(levels) {
			left := levels[:pointer]
			right := levels[pointer+1:]

			new_levels := append([]int{}, left...)
			new_levels = append(new_levels, right...)

			if safe_decreasing(new_levels) || safe_increasing(new_levels) {
				found_safe = true
				break
			}

			pointer++

		}

		if found_safe {
			safe++
		}

	}

	return safe
}
