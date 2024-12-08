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

func point_outside_puzzle(w, h int, p Pos) bool {
	return p.x < 0 || p.x >= h || p.y < 0 || p.y >= w
}

func part1(input []string) int {
	puzzle := make(map[rune][]Pos, 0)
	w, h := len(input[0]), len(input)

	for i, row := range input {
		for j, col := range row {
			if col != '.' {
				puzzle[col] = append(puzzle[col], Pos{i, j})
			}
		}
	}

	anti_nodes := make(map[Pos]int, 0)

	for _, positions := range puzzle {
		if len(positions) < 2 {
			continue
		}

		for i := 0; i < len(positions)-1; i++ {
			for j := i + 1; j < len(positions); j++ {
				p1 := Pos{positions[i].x, positions[i].y}
				p2 := Pos{positions[j].x, positions[j].y}

				diff_x := p1.x - p2.x
				diff_y := p1.y - p2.y

				p3 := Pos{p1.x - 2*diff_x, p1.y - 2*diff_y}
				p4 := Pos{p2.x + 2*diff_x, p2.y + 2*diff_y}

				if !point_outside_puzzle(w, h, p3) {
					anti_nodes[p3] = 0
				}

				if !point_outside_puzzle(w, h, p4) {
					anti_nodes[p4] = 0
				}
			}
		}
	}

	return len(anti_nodes)
}

func part2(input []string) int {
	puzzle := make(map[rune][]Pos, 0)
	w, h := len(input[0]), len(input)

	for i, row := range input {
		for j, col := range row {
			if col != '.' {
				puzzle[col] = append(puzzle[col], Pos{i, j})
			}
		}
	}

	anti_nodes := make(map[Pos]int, 0)

	for _, positions := range puzzle {
		if len(positions) < 2 {
			continue
		}

		for i := 0; i < len(positions)-1; i++ {
			for j := i + 1; j < len(positions); j++ {
				p1 := Pos{positions[i].x, positions[i].y}
				p2 := Pos{positions[j].x, positions[j].y}

				diff_x := p1.x - p2.x
				diff_y := p1.y - p2.y

				k := 0

				p3 := Pos{p1.x - k*diff_x, p1.y - k*diff_y}
				p4 := Pos{p2.x + k*diff_x, p2.y + k*diff_y}

				for !point_outside_puzzle(w, h, p3) ||
					!point_outside_puzzle(w, h, p4) {

					if !point_outside_puzzle(w, h, p3) {
						anti_nodes[p3] = 0
					}

					if !point_outside_puzzle(w, h, p4) {
						anti_nodes[p4] = 0
					}

					k += 1
					p3 = Pos{p1.x - k*diff_x, p1.y - k*diff_y}
					p4 = Pos{p2.x + k*diff_x, p2.y + k*diff_y}
				}
			}
		}
	}

	return len(anti_nodes)
}
