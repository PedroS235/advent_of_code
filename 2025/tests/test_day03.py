import pytest
from aocd.models import Puzzle

from day01.main import parse, part1, part2


@pytest.fixture
def puzzle():
    puzzle = Puzzle(2025, 3)
    return puzzle


@pytest.fixture
def example(puzzle: Puzzle):
    input = puzzle.examples[0].input_data
    return parse(input)


@pytest.fixture
def input(puzzle: Puzzle):
    input = puzzle.input_data
    return parse(input)


def test_part1_example(puzzle: Puzzle, example):
    """Test part 1 on example input."""
    assert part1(example) == puzzle.examples[0].answer_a


def test_part1_input(puzzle: Puzzle, input):
    """Test part 1 on example input."""
    assert part1(input) == puzzle.answer_a


def test_part2_example(puzzle: Puzzle, example):
    """Test part 2 on example input."""
    assert part2(example) == puzzle.examples[0].answer_b


def test_part2_input(puzzle: Puzzle, input):
    """Test part 2 on example input."""
    assert part2(input) == puzzle.answer_b
