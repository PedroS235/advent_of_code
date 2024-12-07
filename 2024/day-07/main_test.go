package main

import (
	"strings"
	"testing"
)

func TestPart1(t *testing.T) {
	input := `190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20`

	// Simulate reading lines from the input
	lines := strings.Split(input, "\n")
	expected := 3749

	result := part1(lines)
	if result != expected {
		t.Errorf("part1() = %d; want %d", result, expected)
	}
}

func TestPart2(t *testing.T) {
	input := `190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20`

	// Simulate reading lines from the input
	lines := strings.Split(input, "\n")
	expected := 11387

	result := part2(lines)
	if result != expected {
		t.Errorf("part2() = %d; want %d", result, expected)
	}
}
