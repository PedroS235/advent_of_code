# aoc_template.py

import pathlib
import sys


def parse(puzzle_input: str):
    """Parse input."""

    combinations = []
    for line in puzzle_input.splitlines():
        dir = line[0]
        n = line[1:]

        combinations.append((dir, n))

    return combinations


def part1(data):
    """Solve part 1."""
    secret = 0
    dial = 50

    for combination in data:
        dir, n = combination

        n = int(n)
        n = n if dir == "R" else -n

        dial = (dial + n) % 100

        if dial == 0:
            secret += 1

    return str(secret)


def part2(data):
    """Solve part 2."""
    secret = 0
    dial = 50

    for combination in data:
        dir, n = combination
        n = int(n)
        new_pos = dial + n if dir == "R" else dial - n

        if new_pos > 99 or new_pos < 0:
            count = abs(new_pos) // 100
            secret += count

        dial = new_pos % 100
        #
        # if dial == 0:
        #     secret += 1

    return str(secret)


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
