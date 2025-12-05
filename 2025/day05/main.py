# aoc_template.py
import operator
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

    return str(fresh)


def included(range1, range2):
    return range1[0] >= range2[0] and range1[1] <= range2[1]


def part2(ranges: dict, available):
    """Solve part 2."""
    numbers = []

    ranges = dict(sorted(ranges.items(), key=operator.itemgetter(0)))
    fresh = 0
    for key1 in ranges.keys():
        skip = False
        range1 = [key1, ranges[key1]]
        original_range = (key1, ranges[key1])
        # print("Looking", range1)

        # First check if original range is fully included in another
        for key2 in ranges.keys():
            range2 = (key2, ranges[key2])
            if key1 == key2:
                continue
            if included(original_range, range2):
                # print("skipped, fully included in", range2)
                skip = True
                break

        if skip:
            # print()
            continue

        # Then adjust for overlaps
        for key2 in ranges.keys():
            range2 = (key2, ranges[key2])
            if key1 == key2:
                continue

            if range1[0] >= range2[0] and range1[0] <= range2[1]:
                # print(f"{range1} -> {range2}")
                range1[0] = range2[1] + 1
                break  # Only adjust based on first overlap found

        if range1[0] > range1[1]:
            skip = True

        if not skip:
            # print("Actual range", range1)
            diff = range1[1] - range1[0] + 1
            fresh += diff
            # print("Diff:", diff)
        # print()
        # fresh += ranges[key1] - key1 + 1
    # print(sorted(numbers))
    # print(len(numbers))

    return str(fresh)


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
