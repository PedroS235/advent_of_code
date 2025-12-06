# aoc_template.py

import pathlib
import sys


def group(rows: list[str], cols_len):
    new_rows = []

    for row in rows:
        i = 0
        col_i = 0
        new_row = []
        col_val = ""
        while i < len(row):
            col_val += row[i] if row[i] != "" else " "
            if len(col_val) == cols_len[col_i]:
                new_row.append(col_val)
                col_i += 1
                col_val = ""
            i += 1

        new_rows.append(new_row)

    return new_rows


def parse_cols(data):
    """Parse input."""

    cols = []
    cols_len = []
    rows = []
    for i, line in enumerate(data):
        rows.append(line.split(" "))

    # Obtain the number of columns per column
    ops = rows[-1]
    found = 0
    for i in range(len(ops)):
        op = ops[i]
        if op != "":
            if i - found == 0:
                continue
            cols_len.append(i - found)
            found = i
    cols_len.append(len(ops) - found)

    rows = group(rows[:-1], cols_len)

    for i, row in enumerate(rows):
        for c in range(len(row)):
            if len(cols) - 1 < c:
                cols.append([row[c]])
                continue
            cols[c].append(row[c])

    return cols, list(filter(lambda x: x != "", ops)), cols_len


def parse(puzzle_input: str):
    """Parse input."""
    return parse_cols(puzzle_input.splitlines())


def part1(problems, ops, _):
    """Solve part 1."""
    total = 0
    for i, col in enumerate(problems):
        op = ops[i]
        eq = ""

        for i, n in enumerate(col):
            if i == len(col) - 1:
                eq += n
            else:
                eq += n + op

        total += eval(eq)

    return str(total)


def part2(problems, ops, cols_len):
    """Solve part 2."""

    total = 0

    for i, problem in enumerate(problems):
        op = ops[i]

        eq = ""

        for j in range(cols_len[i]):
            val = ""
            for k in range(len(problem)):
                val += problem[k][j]

            if j == cols_len[i] - 1:
                eq += val
            else:
                eq += val + op
        total += eval(eq)

    return str(total)


def solve(puzzle_input):
    """Solve the puzzle for the given input."""
    data = parse(puzzle_input)
    solution1 = part1(*data)
    solution2 = part2(*data)

    return solution1, solution2


if __name__ == "__main__":
    for path in sys.argv[1:]:
        print(f"{path}:")
        puzzle_input = pathlib.Path(path).read_text()
        solutions = solve(puzzle_input)
        print("\n".join(str(solution) for solution in solutions))
