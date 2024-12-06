package main

import (
	"bufio"
	"fmt"
	"os"
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

type Pos struct {
	x int
	y int
}

func guard_in_edges(w, h int, pos Pos) bool {
	return pos.x == 0 || pos.x == w-1 || pos.y == 0 || pos.y == h-1
}

func part1(input []string) int {
	puzzle := make(map[Pos]string, 0)
	w, h := len(input[0]), len(input)
	var guard_pos Pos

	for i, row := range input {
		for j, col := range row {
			if col == '^' {
				guard_pos = Pos{i, j}
			}
			puzzle[Pos{i, j}] = string(col)
		}
	}

	dir_idx := 0
	possible_dir := []Pos{{-1, 0}, {0, 1}, {1, 0}, {0, -1}}
	dir := possible_dir[dir_idx]
	visited := make(map[Pos]int, 0)

	for !guard_in_edges(w, h, guard_pos) {
		new_pos := Pos{guard_pos.x + dir.x, guard_pos.y + dir.y}
		if puzzle[new_pos] == "." {
			visited[guard_pos] = 0
			puzzle[guard_pos] = "."
			puzzle[new_pos] = "^"
			guard_pos = new_pos

		} else {
			dir_idx = (dir_idx + 1) % 4
			dir = possible_dir[dir_idx]
		}
	}
	visited[guard_pos] = 0

	return len(visited)
}

func part2(input []string) int {

	puzzle := make(map[Pos]string, 0)
	w, h := len(input[0]), len(input)
	var guard_pos Pos

	for i, row := range input {
		for j, col := range row {
			if col == '^' {
				guard_pos = Pos{i, j}
			}
			puzzle[Pos{i, j}] = string(col)
		}
	}

	dir_idx := 0
	possible_dir := []Pos{{-1, 0}, {0, 1}, {1, 0}, {0, -1}}
	dir := possible_dir[dir_idx]
	visited := make(map[Pos]int, 0)
	obstacles := make(map[Pos]int, 0)

	for !guard_in_edges(w, h, guard_pos) {
		new_pos := Pos{guard_pos.x + dir.x, guard_pos.y + dir.y}
		// fmt.Println("Curr Pos(", guard_pos.x, ",", guard_pos.y, ") -> ", puzzle[guard_pos])
		// fmt.Println("New Pos(", new_pos.x, ",", new_pos.y, ") -> ", puzzle[new_pos])
		if puzzle[new_pos] == "." {
			visited[guard_pos] = 0
			puzzle[guard_pos] = "."
			puzzle[new_pos] = "^"
			guard_pos = new_pos

		} else {
			pos := Pos{new_pos.x - dir.x, new_pos.y - dir.y}
			obstacles[pos] = 0
			dir_idx = (dir_idx + 1) % 4
			dir = possible_dir[dir_idx]
		}
	}
	visited[guard_pos] = 0

	fmt.Println(obstacles)

	return len(visited) + 1
}
