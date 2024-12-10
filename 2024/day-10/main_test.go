package main

import (
	"strings"
	"testing"
)

func TestPart1(t *testing.T) {

	input := `89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732`

	// Simulate reading lines from the input
	lines := strings.Split(input, "\n")
	expected := 36

	result := part1(lines)
	if result != expected {
		t.Errorf("part1() = %d; want %d", result, expected)
	}
}

func TestPart2(t *testing.T) {
	input := `89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732`

	// Simulate reading lines from the input
	lines := strings.Split(input, "\n")
	expected := 81

	result := part2(lines)
	if result != expected {
		t.Errorf("part2() = %d; want %d", result, expected)
	}
}
