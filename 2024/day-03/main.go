package main

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"strconv"
	"unicode"
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

func str_to_int(input string) int {
	var output int

	for i := 0; i < len(input); i++ {
		output, _ = strconv.Atoi(input)
	}

	return output
}

type Muliplication struct {
	a int
	b int
}

func read_number(instructions string, pointer *int) int {

	var number string

	for *pointer < len(instructions) {
		if !unicode.IsDigit(rune(instructions[*pointer])) {
			break
		}

		number += string(instructions[*pointer])
		*pointer++
	}

	return str_to_int(number)
}

func read_multiplication(instructions string, pointer *int) (Muliplication, error) {
	muliplication := Muliplication{}

	if *pointer+4 >= len(instructions) || instructions[*pointer:*pointer+4] != "mul(" {
		return muliplication, errors.New("not a multiplication")
	}
	*pointer += 4

	muliplication.a = read_number(instructions, pointer)

	if instructions[*pointer] != ',' {
		return muliplication, errors.New("missing ,")
	}
	*pointer++

	muliplication.b = read_number(instructions, pointer)

	if instructions[*pointer] != ')' {
		return muliplication, errors.New("missing )")
	}
	*pointer++

	return muliplication, nil
}

func read_dont(instructions string, pointer *int) bool {
	if *pointer+6 <= len(instructions) && instructions[*pointer:*pointer+6] == "don't(" {
		*pointer += 6
		return instructions[*pointer] == ')'
	}
	return false
}

func read_do(instructions string, pointer *int) bool {
	if *pointer+3 <= len(instructions) && instructions[*pointer:*pointer+3] == "do(" {
		*pointer += 3
		return instructions[*pointer] == ')'
	}
	return false
}

func parse2(instructions string, enabled *bool) int {
	sum := 0
	curr := 0

	for curr < len(instructions) {
		if read_dont(instructions, &curr) {
			*enabled = false
			curr++
			continue
		}
		if read_do(instructions, &curr) {
			*enabled = true
			curr++
			continue
		}

		if *enabled {
			m, ret := read_multiplication(instructions, &curr)
			if ret == nil {
				sum += m.a * m.b
				continue
			}
		}
		curr++
	}

	return sum
}

func parse1(instructions string) int {
	sum := 0
	curr := 0

	for curr < len(instructions) {
		m, ret := read_multiplication(instructions, &curr)
		if ret == nil {
			sum += m.a * m.b
			continue
		}
		curr++
	}

	return sum
}

func part1(input []string) int {

	sum := 0

	for _, line := range input {
		sum += parse1(line)
	}

	return sum
}

func part2(input []string) int {
	enabled := true
	sum := 0

	for _, line := range input {
		res := parse2(line, &enabled)
		sum += res
	}

	return sum
}
