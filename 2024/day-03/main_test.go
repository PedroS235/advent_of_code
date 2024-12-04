package main

import (
	"strings"
	"testing"
)

func TestPart1(t *testing.T) {
	input := `xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))`

	// Simulate reading lines from the input
	lines := strings.Split(input, "\n")
	expected := 161

	result := part1(lines)
	if result != expected {
		t.Errorf("part1() = %d; want %d", result, expected)
	}
}

func TestPart2(t *testing.T) {
	input := `xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))`

	// Simulate reading lines from the input
	lines := strings.Split(input, "\n")
	expected := 48

	result := part2(lines)
	if result != expected {
		t.Errorf("part2() = %d; want %d", result, expected)
	}
}
