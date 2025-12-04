# aoc_template.py

import pathlib
import sys


def parse(puzzle_input: str):
    """Parse input."""

    rolls = set()

    lines = puzzle_input.splitlines()

    for row, line in enumerate(lines):
        for col, p in enumerate(line):
            if p == "@":
                rolls.add((int(row), int(col)))

    return rolls


def part1(data: set[tuple[int, int]]):
    """Solve part 1."""

    positions = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (1, 1), (1, -1), (-1, 1)]

    valid_rolls = 0
    for key in data:
        count = 0
        for pos in positions:
            new_pos = (key[0] + pos[0], key[1] + pos[1])

            if new_pos in data:
                count += 1
                if count >= 4:
                    break

        if count < 4:
            valid_rolls += 2

    return str(valid_rolls)


def part2(data):
    """Solve part 2."""


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
