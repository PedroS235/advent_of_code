package main

import (
	"strings"
	"testing"
)

func TestPart1(t *testing.T) {
	input := `7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9`

	// Simulate reading lines from the input
	lines := strings.Split(input, "\n")
	expected := 2

	result := part1(lines)
	if result != expected {
		t.Errorf("part1() = %d; want %d", result, expected)
	}
}

func TestPart2(t *testing.T) {
	input := `7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9`

	// Simulate reading lines from the input
	lines := strings.Split(input, "\n")
	expected := 4

	result := part2(lines)
	if result != expected {
		t.Errorf("part1() = %d; want %d", result, expected)
	}
}
