# aoc_template.py

import pathlib
import sys


def parse(puzzle_input: str):
    """Parse input."""
    id_ranges = []
    for id_range in puzzle_input.split(","):
        id_ranges.append([int(x) for x in id_range.split("-")])

    return id_ranges


def generate_ids_in_range(min_id, max_id):
    current_id = min_id

    while current_id <= max_id:
        yield str(current_id)
        current_id += 1


def part1(data):
    """Solve part 1."""
    sum_of_matching_ids = 0

    for min_id, max_id in data:
        ids = generate_ids_in_range(min_id, max_id)

        for id in ids:
            if len(id) % 2 != 0:
                continue

            left_half, right_half = id[: len(id) // 2], id[len(id) // 2 :]
            sum_of_matching_ids += int(id) if left_half == right_half else 0

    return str(sum_of_matching_ids)


def has_repeating_pattern(id: str):
    if len(id) < 2:
        return False

    prefix_length = 1

    prefix = id[0:prefix_length]
    suffix = id[prefix_length:]

    while len(prefix) <= len(suffix) and prefix_length < len(id):
        if is_repeating_pattern(prefix, suffix):
            return True
        prefix_length += 1
        prefix = id[0:prefix_length]
        suffix = id[prefix_length:]

    return False


def is_repeating_pattern(pattern, text):
    position = 0
    while len(pattern) <= len(text) and position + len(pattern) <= len(text):
        segment = text[position : position + len(pattern)]
        if pattern != segment:
            return False
        position += len(pattern)

    return position == len(text)


def part2(data):
    """Solve part 2."""
    sum_of_matching_ids = 0

    for min_id, max_id in data:
        ids = generate_ids_in_range(min_id, max_id)

        for id in ids:
            is_valid = has_repeating_pattern(id)
            sum_of_matching_ids += int(id) if is_valid else 0

    return str(sum_of_matching_ids)


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
