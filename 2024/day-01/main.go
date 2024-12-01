package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
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

func abs(x int) int {
	if x < 0 {
		return -x
	}

	return x
}

func part1(input []string) int {
	answer := 0

	var left_col []string
	var right_col []string

	for _, line := range input {
		pair := strings.Split(line, " ")

		left_col = append(left_col, pair[0])
		right_col = append(right_col, pair[len(pair)-1])
	}

	sort.Strings(left_col)
	sort.Strings(right_col)

	for i, item := range left_col {

		left, err := strconv.Atoi(item)

		if err != nil {
			fmt.Println("Failed to convert string to int")
			return answer
		}

		right, err := strconv.Atoi(right_col[i])

		if err != nil {
			fmt.Println("Failed to convert string to int")
			return answer
		}

		answer = answer + abs(left-right)

	}

	return answer
}

func part2(input []string) int {
	answer := 0

	var left_col []string
	var right_col map[string]int
	right_col = make(map[string]int)

	for _, line := range input {
		pair := strings.Split(line, " ")

		left_col = append(left_col, pair[0])

		val, exists := right_col[pair[len(pair)-1]]
		if exists {
			right_col[pair[len(pair)-1]] = val + 1
		} else {
			right_col[pair[len(pair)-1]] = 1
		}
	}

	for _, item := range left_col {

		freq := right_col[item]

		left, err := strconv.Atoi(item)

		if err != nil {
			fmt.Println("Failed to convert string to int")
			return answer
		}

		answer = answer + left*freq

	}

	return answer
}
