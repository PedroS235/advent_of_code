# aoc_template.py

import pathlib
import sys


def parse(puzzle_input: str):
    """Parse input."""

    ranges = dict()
    available = []

    switch = False
    for line in puzzle_input.splitlines():
        if line == "":
            switch = True
            continue

        if switch:
            available.append(int(line))
        else:
            lower, upper = list(map(int, line.split("-")))

            if ranges.get(lower):
                if ranges[lower] < upper:
                    ranges[lower] = upper
            else:
                ranges[lower] = upper
    return ranges, available


def part1(ranges: dict, available):
    """Solve part 1."""

    # print(ranges)
    # print(available)
    fresh = 0
    for id in available:
        for key in ranges.keys():
            if id >= key and id <= ranges[key]:
                fresh += 1
                break

def part2(data):
    """Solve part 2."""


def solve(puzzle_input):
    """Solve the puzzle for the given input."""
    data = parse(puzzle_input)
    solution1 = part1(*data)
    solution2 = part2(*data)

    return solution1, solution2


if __name__ == "__main__":
    for path in sys.argv[1:]:
        print(f"{path}:")
        puzzle_input = pathlib.Path(path).read_text().strip()
        solutions = solve(puzzle_input)
        print("\n".join(str(solution) for solution in solutions))
