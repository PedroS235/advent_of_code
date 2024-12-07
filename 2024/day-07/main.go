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

type Node struct {
	left  *Node
	right *Node
	val   int
}

type Equation struct {
	result int
	terms  []int
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

func eval_equation(target int, value int, terms []int) bool {
	if len(terms) == 0 {
		if value == target {
			return true
		}
		return false
	}
	if value > target {
		return false
	}

	return eval_equation(target, value+terms[0], terms[1:]) || eval_equation(target, value*terms[0], terms[1:])
}

func part1(input []string) int {

	answer := 0

	for _, line := range input {
		equation_fields := strings.Split(line, ": ")
		result, _ := strconv.Atoi(equation_fields[0])
		terms := convert_str_int(strings.Fields(equation_fields[1]))
		equation := Equation{result, terms}

		if eval_equation(equation.result, equation.terms[0], equation.terms[1:]) {
			answer += equation.result
		}
	}

	return answer
}

func eval_equation2(target int, value int, terms []int) bool {
	if len(terms) == 0 {
		if value == target {
			return true
		}
		return false
	}
	if value > target {
		return false
	}

	concat, _ := strconv.Atoi(fmt.Sprintf("%d%d", value, terms[0]))

	return eval_equation2(target, value+terms[0], terms[1:]) ||
		eval_equation2(target, value*terms[0], terms[1:]) ||
		eval_equation2(target, concat, terms[1:])
}

func part2(input []string) int {

	answer := 0

	for _, line := range input {
		equation_fields := strings.Split(line, ": ")
		result, _ := strconv.Atoi(equation_fields[0])
		terms := convert_str_int(strings.Fields(equation_fields[1]))
		equation := Equation{result, terms}

		if eval_equation2(equation.result, equation.terms[0], equation.terms[1:]) {
			answer += equation.result
		}

	}

	return answer
}
