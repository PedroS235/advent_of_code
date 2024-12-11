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
	fmt.Println("Result1:", result1)
	result2 := part2(inputLines)
	fmt.Println("Result2:", result2)
}

func part1(input []string) int {
	// On first try I did the straight forward solution using lists
	fields := strings.Fields(input[0])

	stones := make([]int, len(fields))
	for i, field := range fields {
		number, _ := strconv.Atoi(field)
		stones[i] = number
	}
	total := 0

	cache := make(map[string]int)

	for _, stone := range stones {
		total += count(stone, 25, cache)
	}

	return total
}

func count(stone, steps int, cache map[string]int) int {
	key := fmt.Sprintf("%d,%d", stone, steps)
	if steps == 0 {
		return 1
	}

	if result, ok := cache[key]; ok {
		return result
	}

	if stone == 0 {
		cache[key] = count(1, steps-1, cache)
		return cache[key]
	}

	digits := strconv.Itoa(stone)

	if len(digits)%2 == 0 {
		lhs, _ := strconv.Atoi(digits[:len(digits)/2])
		rhs, _ := strconv.Atoi(digits[len(digits)/2:])

		cache[key] = count(lhs, steps-1, cache) + count(rhs, steps-1, cache)
		return cache[key]
	}
	cache[key] = count(stone*2024, steps-1, cache)
	return cache[key]
}

func part2(input []string) int {
	// Based myself on Hyperneutrino solution
	fields := strings.Fields(input[0])

	stones := make([]int, len(fields))
	for i, field := range fields {
		number, _ := strconv.Atoi(field)
		stones[i] = number
	}
	total := 0

	cache := make(map[string]int)

	for _, stone := range stones {
		total += count(stone, 75, cache)
	}

	return total
}
