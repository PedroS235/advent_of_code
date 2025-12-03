# aoc_template.py

import pathlib
import sys


def parse(puzzle_input: str):
    """Parse input."""
    banks = []
    for bank in puzzle_input.splitlines():
        banks.append(bank)

    return banks


def pick_k_digits(bank: str, size: int) -> str:
    print("bank: ", bank)
    print("len: ", len(bank))
    to_remove = len(bank) - size
    stack = []

    for digit in bank:
        print("outer", stack)
        while to_remove and stack and stack[-1] < digit:
            print("inner", stack, "size:", to_remove)
            stack.pop()
            to_remove -= 1
        stack.append(digit)

    return "".join(stack[:size])


def part1(banks: list[str]):
    """Solve part 1."""
    total_joltage = 0

    for bank in banks:
        v = pick_k_digits(bank, 2)
        total_joltage += int(v)

    return str(total_joltage)


def part2(banks):
    """Solve part 2."""
    total_joltage = 0

    for bank in banks:
        v = pick_k_digits(bank, 12)
        total_joltage += int(v)

    return str(total_joltage)


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
