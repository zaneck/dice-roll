from pydantic import BaseModel


class SimpleRollResponseModel(BaseModel):
    dice_sides: int
    result: int
