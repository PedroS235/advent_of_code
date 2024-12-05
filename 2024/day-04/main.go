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

func reverse(str string) (result string) {
	for _, v := range str {
		result = string(v) + result
	}
	return
}

func validate_diag(grid []string, x, y int) int {
	directions := [][2]int{{1, 1}, {1, -1}, {-1, 1}, {-1, -1}}
	result := 0

	for _, dir := range directions {
		diag := ""
		for i := 0; i < 4; i++ {
			dx, dy := x+dir[0]*i, y+dir[1]*i
			if dx < 0 || dx >= len(grid) || dy < 0 || dy >= len(grid[0]) {
				break
			}
			diag += string(grid[dx][dy])
		}
		if diag == "XMAS" {
			result++
		}
	}
	return result
}

func validate_ver(grid []string, x, y int) int {
	result := 0
	vertical := func(start, step int) string {
		ver := ""
		for i := 0; i < 4; i++ {
			dx := start + i*step
			if dx < 0 || dx >= len(grid) {
				break
			}
			ver += string(grid[dx][y])
		}
		return ver
	}

	if vertical(x, 1) == "XMAS" {
		result++
	}
	if vertical(x, -1) == "XMAS" {
		result++
	}
	return result
}

func validate_hor(grid []string, x, y int) int {
	var right string
	var left string
	result := 0

	if y <= len(grid[0])-4 {
		right = grid[x][y : y+4]
	}

	if y >= 3 {
		left = reverse(grid[x][y-3 : y+1])
	}

	if right == "XMAS" {
		result++
	}
	if left == "XMAS" {
		result++
	}
	return result
}

func valid_xmas(grid []string, x, y int) int {
	return validate_hor(grid, x, y) + validate_ver(grid, x, y) + validate_diag(grid, x, y)
}

func validate_mas(grid []string, x, y int) int {
	result := 0

	if x < 1 || x > len(grid)-2 || y < 1 || y > len(grid[0])-2 {
		return 0
	}

	diag1 := string(grid[x-1][y-1])
	diag1 += string(grid[x][y])
	diag1 += string(grid[x+1][y+1])

	diag2 := string(grid[x+1][y-1])
	diag2 += string(grid[x][y])
	diag2 += string(grid[x-1][y+1])

	if diag1 == "MAS" || diag1 == "SAM" {
		result++
	}

	if diag2 == "MAS" || diag2 == "SAM" {
		result++
	}

	if result == 2 {
		return 1
	}
	return 0
}

func part1(input []string) int {

	var grid []string
	xmas_words := 0

	for _, line := range input {
		grid = append(grid, line)
	}

	for i, row := range grid {
		for j, col := range row {
			if col != 'X' {
				continue
			}
			xmas_words += valid_xmas(grid, i, j)
		}
	}

	return xmas_words
}

func part2(input []string) int {
	var grid []string
	xmas_words := 0

	for _, line := range input {
		grid = append(grid, line)
	}

	for i, row := range grid {
		for j, col := range row {
			if col != 'A' {
				continue
			}
			xmas_words += validate_mas(grid, i, j)
		}
	}

	return xmas_words
}
