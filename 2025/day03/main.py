# aoc_template.py

import pathlib
import sys


def parse(puzzle_input: str):
    """Parse input."""
    banks = []
    for bank in puzzle_input.splitlines():
        banks.append(bank)

    return banks


def part1(banks: list[list[str]]):
    """Solve part 1."""
    total_joltage = 0

    for bank in banks:
        max_joltage = 0
        for i in range(len(bank) - 1):
            for j in range(i + 1, len(bank)):
                joltage = int(bank[i] + bank[j])
                if joltage > max_joltage:
                    max_joltage = joltage
        total_joltage += max_joltage

    return str(total_joltage)


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
