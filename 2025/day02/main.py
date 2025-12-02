# aoc_template.py

import pathlib
import sys


def parse(puzzle_input: str):
    """Parse input."""
    ids = []
    for id_range in puzzle_input.split(","):
        ids.append([int(x) for x in id_range.split("-")])

    return ids


def gen_range(min_id, max_id):
    count = min_id

    while count <= max_id:
        yield str(count)
        count += 1


def part1(data):
    """Solve part 1."""
    invalid_ids = 0

    for product_range in data:
        ids = gen_range(product_range[0], product_range[1])

        for id in ids:
            if len(id) % 2 != 0:
                continue

            left, right = id[: len(id) // 2], id[len(id) // 2 :]
            invalid_ids += int(id) if left == right else 0

    return str(invalid_ids)


def investiate_id(id: str):
    if len(id) < 2:
        return False

    end = 1

    head = id[0:end]
    tail = id[end:]

    while len(head) <= len(tail) and end < len(id):
        if repeats(head, tail):
            return True
        end += 1
        head = id[0:end]
        tail = id[end:]

    return False


def repeats(head, tail):
    idx = 0
    while len(head) <= len(tail) and idx + len(head) <= len(tail):
        cmp = tail[idx : idx + len(head)]
        if head != cmp:
            return False
        idx += len(head)

    if idx + len(head) > len(tail) and idx < len(tail):
        return False
    return True


def part2(data):
    """Solve part 2."""
    invalid_ids = 0

    for product_range in data:
        ids = gen_range(product_range[0], product_range[1])

        for id in ids:
            valid = investiate_id(id)
            invalid_ids += int(id) if valid else 0

    return str(invalid_ids)


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
