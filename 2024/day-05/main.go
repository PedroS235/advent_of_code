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

func convert_str_int(input []string) []int {
	output := make([]int, len(input))
	for i, n := range input {

		converted, _ := strconv.Atoi(n)
		// We should check for error. Leaving it aside
		output[i] = converted
	}

	return output
}

func read_input(input []string) (map[int][]int, [][]int) {
	found_end_page_rules := false
	rules := make(map[int][]int)
	var updates [][]int

	for _, line := range input {
		if line == "" {
			found_end_page_rules = true
			continue
		}

		if found_end_page_rules {

			update := strings.Split(line, ",")
			updates = append(updates, convert_str_int(update))
			continue
		}

		rule := strings.Split(line, "|")

		rule1, ret := strconv.Atoi(rule[0])
		if ret != nil {
			fmt.Println("Failed to convert rule1 digit")
		}

		rule2, ret := strconv.Atoi(rule[1])
		if ret != nil {
			fmt.Println("Failed to convert rule2 digit")
		}

		values, ok := rules[rule1]

		if !ok {
			rules[rule1] = make([]int, 1)
			rules[rule1] = append(rules[rule1], rule2)
		}
		rules[rule1] = append(values, rule2)
	}

	return rules, updates
}
func contains(slice []int, value int) bool {
	for _, v := range slice {
		if v == value {
			return true
		}
	}
	return false
}

func is_valid_update(rules map[int][]int, update []int) bool {
	var seen []int

	for _, rule := range update {
		future := rules[rule]
		for _, x := range future {
			if contains(seen, x) {
				return false
			}
		}
		seen = append(seen, rule)
	}

	return true
}

func find_middle_element(input []int) int {
	middle := len(input) / 2

	return input[middle]
}

func part1(input []string) int {
	result := 0

	rules, updates := read_input(input)

	for _, update := range updates {
		if is_valid_update(rules, update) {
			result += find_middle_element(update)
		}
	}

	return result
}

func fix_update(rules map[int][]int, update []int) []int {
	if is_valid_update(rules, update) {
		return update
	}

	var seen []int

	for i, rule := range update {
		future := rules[rule]
		for _, x := range future {
			if contains(seen, x) {
				update[i], update[i-1] = update[i-1], update[i]
				return fix_update(rules, update)
			}
		}
		seen = append(seen, rule)
	}

	return update
}

func part2(input []string) int {
	result := 0
	rules, updates := read_input(input)

	for _, update := range updates {
		if !is_valid_update(rules, update) {
			fixed_update := fix_update(rules, update)
			result += find_middle_element(fixed_update)
		}
	}

	return result
}
