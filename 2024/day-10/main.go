package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
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

func inside_map(r, c, w, h int) bool {
	return r >= 0 && r < w && c >= 0 && c < h
}

type Pos struct {
	r, c int
}

func follow_trail_score(input []string, start Pos, graph map[Pos][][]int, visited map[Pos]int) int {
	if input[start.r][start.c] == '9' {
		return 1
	}
	neighbours := graph[start]
	trails := 0

	for _, neighbour := range neighbours {
		height, _ := strconv.Atoi(string(input[start.r][start.c]))
		if neighbour[0] != height+1 {
			continue
		}

		next := Pos{neighbour[1], neighbour[2]}

		_, ok := visited[next]
		if ok {
			continue
		}
		visited[next] = 0

		trails += follow_trail_score(input, next, graph, visited)
	}
	return trails
}

func follow_trail_rating(input []string, start Pos, graph map[Pos][][]int) int {
	if input[start.r][start.c] == '9' {
		return 1
	}
	neighbours := graph[start]
	trails := 0

	for _, neighbour := range neighbours {
		height, _ := strconv.Atoi(string(input[start.r][start.c]))
		if neighbour[0] != height+1 {
			continue
		}

		next := Pos{neighbour[1], neighbour[2]}

		trails += follow_trail_rating(input, next, graph)
	}
	return trails
}

func part1(input []string) int {
	score := 0
	w, h := len(input), len(input[0])
	graph := make(map[Pos][][]int)
	var starts []Pos

	dirs := [][]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}

	for r, row := range input {
		for c, height_str := range row {
			height, _ := strconv.Atoi(string(height_str))

			// Is in an edge
			if height == 0 {
				starts = append(starts, Pos{r, c})
			}

			neighbours := make([][]int, 0)
			for _, dir := range dirs {
				diff_r, diff_c := r+dir[0], c+dir[1]

				if inside_map(diff_r, diff_c, w, h) {
					n_height, _ := strconv.Atoi(string(input[diff_r][diff_c]))
					neighbours = append(neighbours, []int{n_height, diff_r, diff_c})
				}
			}
			graph[Pos{r, c}] = neighbours
		}
	}

	for _, start := range starts {
		visited := make(map[Pos]int)
		visited[start] = 0
		score += follow_trail_score(input, start, graph, visited)
	}

	return score
}

func part2(input []string) int {
	score := 0
	w, h := len(input), len(input[0])
	graph := make(map[Pos][][]int)
	var starts []Pos

	dirs := [][]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}

	for r, row := range input {
		for c, height_str := range row {
			height, _ := strconv.Atoi(string(height_str))

			// Is in an edge
			if height == 0 {
				starts = append(starts, Pos{r, c})
			}

			neighbours := make([][]int, 0)
			for _, dir := range dirs {
				diff_r, diff_c := r+dir[0], c+dir[1]

				if inside_map(diff_r, diff_c, w, h) {
					n_height, _ := strconv.Atoi(string(input[diff_r][diff_c]))
					neighbours = append(neighbours, []int{n_height, diff_r, diff_c})
				}
			}
			graph[Pos{r, c}] = neighbours
		}
	}

	for _, start := range starts {
		score += follow_trail_rating(input, start, graph)
	}

	return score
}
