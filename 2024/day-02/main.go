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

	result := part2(inputLines)
	fmt.Println("Result:", result)
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

		if safe_decreasing_tolerance(levels) || safe_increasing_tolerance(levels) {
			safe++
		}
	}

	return safe
}

func safe_increasing_tolerance(reports []int) bool {
	tolerance := 1
	p1, p2 := 0, 1
	for p2 < len(reports) {
		diff := reports[p2] - reports[p1]
		if diff <= 0 || diff > 3 {
			if tolerance == 0 {
				return false
			}

			tolerance--
			if p1 == 0 {
				p1++
				p2++
				continue
			}
			p2++
			continue
		}

		p1, p2 = p2, p2+1
	}
	return true
}

func safe_decreasing_tolerance(reports []int) bool {
	tolerance := 1
	p1, p2 := 0, 1
	for p2 < len(reports) {
		diff := reports[p1] - reports[p2]
		if diff <= 0 || diff > 3 {
			if tolerance == 0 {
				return false
			}

			tolerance--
			if p1 == 0 {
				p1++
				p2++
				continue
			}

			p2++
			continue
		}

		p1, p2 = p2, p2+1
	}
	return true
}
