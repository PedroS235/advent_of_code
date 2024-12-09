package main

import (
	"strings"
	"testing"
)

func TestPart1(t *testing.T) {
	input := `2333133121414131402`

	// Simulate reading lines from the input
	lines := strings.Split(input, "\n")
	expected := 1928

	result := part1(lines)
	if result != expected {
		t.Errorf("part1() = %d; want %d", result, expected)
	}
}

func TestPart2(t *testing.T) {
	input := `2333133121414131402`

	// Simulate reading lines from the input
	lines := strings.Split(input, "\n")
	expected := 2858

	result := part2(lines)
	if result != expected {
		t.Errorf("part2() = %d; want %d", result, expected)
	}
}
