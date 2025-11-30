# aoc_template.py

import pathlib
import sys
from collections import Counter


def parse(puzzle_input: str):
    """Parse input."""

    left = []
    right = []

    for line in puzzle_input.splitlines():
        l, r = line.split("  ")
        left.append(l)
        right.append(r)

    return left, right


def part1(data):
    """Solve part 1."""
    left, right = data

    left = list(map(int, left))
    right = list(map(int, right))

    left.sort()
    right.sort()

    distances = []
    for l, r in zip(left, right):
        distances.append(abs(l - r))

    return str(sum(distances))


def part2(data):
    """Solve part 2."""
    left, right = data

    left = list(map(int, left))
    right = list(map(int, right))

    r_counter = Counter(right)

    score = 0
    for v in left:
        count = r_counter.get(v, 0)
        score += count * v

    return str(score)


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
