package main

import (
	"strings"
	"testing"
)

func TestPart1(t *testing.T) {
	input := `....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...`

	// Simulate reading lines from the input
	lines := strings.Split(input, "\n")
	expected := 41

	result := part1(lines)
	if result != expected {
		t.Errorf("part1() = %d; want %d", result, expected)
	}
}

func TestPart2(t *testing.T) {
	input := `....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...`

	// Simulate reading lines from the input
	lines := strings.Split(input, "\n")
	expected := 6

	result := part2(lines)
	if result != expected {
		t.Errorf("part2() = %d; want %d", result, expected)
	}
}
