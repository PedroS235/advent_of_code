package main

import (
	"strings"
	"testing"
)

func TestPart1(t *testing.T) {
	input := `3 4
4 3
2 5
1 3
3 9
3 3`

	// Simulate reading lines from the input
	lines := strings.Split(input, "\n")
	expected := 11

	result := part1(lines)
	if result != expected {
		t.Errorf("part1() = %d; want %d", result, expected)
	}
}

func TestPart2(t *testing.T) {
	input := `3 4
4 3
2 5
1 3
3 9
3 3`

	// Simulate reading lines from the input
	lines := strings.Split(input, "\n")
	expected := 31

	result := part2(lines)
	if result != expected {
		t.Errorf("part1() = %d; want %d", result, expected)
	}
}
