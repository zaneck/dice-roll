from pydantic import BaseModel


class MultiRollResponseModel(BaseModel):
    dice_sides: int
    results: list[int]
    min_value: int
    max_value: int
    average: float
    median: float
    total: int
