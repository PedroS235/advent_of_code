# aoc_template.py

import pathlib
import sys


def group(row: list[str]):
    print(row)
    number = ""
    numbers = []
    for i, n in enumerate(row):
        number += n
        if n == "":
            number += " "
            continue

        numbers.append(number)
        number = ""

    return numbers


def parse_cols(data):
    """Parse input."""

    cols = []
    for i, line in enumerate(data):
        row = group(line.rsplit(" "))
        for c in range(len(row)):
            if len(cols) - 1 < c:
                cols.append([row[c]])
                continue
            cols[c].append(row[c])

    return cols


def parse(puzzle_input: str):
    """Parse input."""
    return parse_cols(puzzle_input.splitlines())


def part1(data):
    """Solve part 1."""
    total = 0
    for col in data:
        op = col[-1]
        eq = ""

        for i, n in enumerate(col[:-1]):
            if i < len(col) - 2:
                eq += n + op
            else:
                eq += n

        total += eval(eq)

    return str(total)


def find_max_len(problem):
    max_len = 0
    for p in problem:
        count = 0
        for n in p:
            if n.isdigit():
                count += 1
        max_len = max(max_len, count)

    return max_len


def part2(data):
    """Solve part 2."""
    return
    print(data)

    for problem in data:
        op = problem[-1]
        problem = problem[:-1]
        max_len = find_max_len(problem)

        print(problem, "->", op)
        for j in range(1, max_len + 1):
            t = ""
            for k in range(len(problem)):
                print("l", problem[k])
                if j <= len(problem[k]):
                    t += problem[k][-j]
                else:
                    t += ""  # problem[k]
            print(t)
            print()


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
