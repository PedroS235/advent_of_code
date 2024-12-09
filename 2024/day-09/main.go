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

func part1(input []string) int {
	checksum := 0
	var fs []string

	id := 0

	for _, disk_map := range input {
		for i := 0; i < len(disk_map)-1; i += 2 {
			layout, _ := strconv.Atoi(string(disk_map[i]))
			free_space, _ := strconv.Atoi(string(disk_map[i+1]))

			for range layout {
				fs = append(fs, string(strconv.Itoa(id)))
			}

			for range free_space {
				fs = append(fs, ".")
			}
			id++
		}
		layout, _ := strconv.Atoi(string(disk_map[len(disk_map)-1]))

		for range layout {
			fs = append(fs, string(strconv.Itoa(id)))
		}
	}

	left, right := 0, len(fs)-1

	for left < right {
		if fs[left] != "." {
			left++
			continue
		}
		if fs[right] == "." {
			right--
			continue
		}
		fs[left] = fs[right]
		fs[right] = "."
		right--
		left++
	}

	for pos, id := range fs {
		if id == "." {
			break
		}
		val, _ := strconv.Atoi(id)

		checksum += pos * val
	}

	return checksum
}

func part2(input []string) int {
	checksum := 0
	var fs []string

	id := 0

	for _, disk_map := range input {
		for i := 0; i < len(disk_map)-1; i += 2 {
			layout, _ := strconv.Atoi(string(disk_map[i]))
			free_space, _ := strconv.Atoi(string(disk_map[i+1]))

			for range layout {
				fs = append(fs, string(strconv.Itoa(id)))
			}

			for range free_space {
				fs = append(fs, ".")
			}
			id++
		}
		layout, _ := strconv.Atoi(string(disk_map[len(disk_map)-1]))

		for range layout {
			fs = append(fs, string(strconv.Itoa(id)))
		}
	}

	block_s, block_e := len(fs)-2, len(fs)-1
	seen := make(map[string]int)

	for block_s > 2 {
		free_s, free_e := 0, 1

		// Find block
		for fs[block_e] == "." {
			block_s--
			block_e--
		}

		for fs[block_s] == fs[block_e] {
			block_s--
		}

		_, ok := seen[fs[block_e]]

		for ok {
			block_e = block_s
			block_s--
			for fs[block_e] == "." && block_s > 0 {
				block_s--
				block_e--
			}
			for fs[block_s] == fs[block_e] && block_s > 0 {
				block_s--
			}
			_, ok = seen[fs[block_e]]
			if !ok {
				break
			}
		}

		block := fs[block_s+1 : block_e+1]
		seen[block[0]] = 0

		for free_e < block_s {

			for fs[free_s] != "." {
				free_s++
				free_e++
			}

			for fs[free_s] == fs[free_e] {
				free_e++
			}

			diff := block_e - block_s
			free_space := free_e - free_s

			if diff > free_space {
				free_s = free_e
				free_e++

			} else {
				for i := range diff {
					fs[free_s+i] = block[i]
					fs[block_s+i+1] = "."
				}
				break

			}

		}

		block_e = block_s
		block_s--

	}

	for pos, id := range fs {
		if id == "." {
			continue
		}
		val, _ := strconv.Atoi(id)

		checksum += pos * val
	}

	return checksum
}
