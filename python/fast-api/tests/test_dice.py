import pytest

from dice.dice import Dice


def test_dice_init() -> None:
    d = Dice(6)
    assert d.sides == 6


def test_dice_repr() -> None:
    d = Dice(10)
    assert repr(d) == "Dice(sides=10)"


def test_dice_roll_within_range() -> None:
    d = Dice(20)
    results = [d.roll() for _ in range(100)]
    assert all(1 <= result <= 20 for result in results)


def test_dice_roll_randomness() -> None:
    d = Dice(6)
    rolls = set(d.roll() for _ in range(100))
    # With 100 rolls, should see most faces
    assert rolls.issubset(set(range(1, 7)))
    assert len(rolls) > 1


@pytest.mark.parametrize("sides", [1, 2, 100])
def test_dice_roll_parametrized(sides: int) -> None:
    d = Dice(sides)
    for _ in range(10):
        result = d.roll()
        assert 1 <= result <= sides
