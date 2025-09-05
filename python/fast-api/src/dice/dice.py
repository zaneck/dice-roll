import secrets


class Dice:
    def __init__(self, sides: int) -> None:
        self.sides = sides

    def roll(self) -> int:
        return secrets.randbelow(self.sides) + 1

    def __repr__(self) -> str:
        return f"Dice(sides={self.sides})"
