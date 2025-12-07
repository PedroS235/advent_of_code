# aoc_template.py

import pathlib
import sys


def parse(puzzle_input: str):
    """Parse input."""
    return puzzle_input.splitlines()


def part1(data):
    """Solve part 1."""

    start_col = 0

    for j, col in enumerate(data[0]):
        if col == "S":
            start_col = j
            break

    splitters = set()
    splitters.add(start_col)
    total = 1

    for row in data[1:]:
        for j, col in enumerate(row):
            if col == "^" and j in splitters:
                total += 1
                splitters.remove(j)
                splitters.add(j - 1)
                splitters.add(j + 1)

    return str(total - 1)


def find_timelines(data: list[list[str]], row: int, col: int, cache=dict()):
    if (row, col) in cache:
        return cache[(row, col)]
    if row == len(data):
        return 1  # set([col])

    if data[row][col] != "^":
        return find_timelines(data, row + 1, col)
    right = find_timelines(data, row + 1, col + 1)
    left = find_timelines(data, row + 1, col - 1)
    cache[(row, col)] = right + left
    return cache[(row, col)]


def part2(data):
    """Solve part 2."""
    start_col = 0
    for j, col in enumerate(data[0]):
        if col == "S":
            start_col = j
            break
    return str(find_timelines(data, 1, start_col))


def solve(puzzle_input):
    """Solve the puzzle for the given input."""
    data = parse(puzzle_input)
    solution1 = part1(data)
    solution2 = part2(data)

    return solution1, solution2


if __name__ == "__main__":
    for path in sys.argv[1:]:
        print(f"{path}:")
        puzzle_input = pathlib.Path(path).read_text().strip()
        solutions = solve(puzzle_input)
        print("\n".join(str(solution) for solution in solutions))
