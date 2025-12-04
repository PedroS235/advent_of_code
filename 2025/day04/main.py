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


def count_adjacent(rolls, roll):
    positions = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (1, 1), (1, -1), (-1, 1)]
    count = 0

    for pos in positions:
        new_pos = (roll[0] + pos[0], roll[1] + pos[1])

        if new_pos in rolls:
            count += 1

    return count


def part1(data: set[tuple[int, int]]):
    """Solve part 1."""

    valid_rolls = 0
    for key in data:
        count = count_adjacent(data, key)
        if count < 4:
            valid_rolls += 1

    return str(valid_rolls)


def part2(data: set):
    """Solve part 2."""
    valid_rolls = 0
    prev_len = 0
    while len(data) != prev_len:
        to_remove = []
        for key in data:
            count = count_adjacent(data, key)
            if count < 4:
                to_remove.append(key)

        prev_len = len(data)
        for key in to_remove:
            valid_rolls += 1
            data.remove(key)

    return str(valid_rolls)


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
