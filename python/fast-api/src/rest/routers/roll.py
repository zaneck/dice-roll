from statistics import mean, median

from fastapi import APIRouter

from dice.dice import Dice
from dice.entities.multiroll import MultiRollResponseModel
from dice.entities.simpleroll import SimpleRollResponseModel

router = APIRouter(tags=["Roll"])


@router.get(
    "/roll",
    response_model=SimpleRollResponseModel,
    responses={
        200: {
            "description": "Successful Response",
            "content": {
                "application/json": {"example": {"dice_sides": 6, "result": 4}}
            },
        }
    },
)
async def roll_dice(sides: int = 6) -> SimpleRollResponseModel:
    """Roll a dice with a specified number of sides.

    Parameters
    ----------
    sides : int, optional
        The number of sides on the dice (default is 6).

    Returns
    -------
        A dictionary containing the result of the dice roll.

    Example
    -------
    ```
        GET /roll?sides=6
        Response: {"dice_sides": 6, "result": 4}
    ```

    """

    dice = Dice(sides)
    result = dice.roll()
    return SimpleRollResponseModel(dice_sides=sides, result=result)


@router.get(
    "/multi-roll",
    response_model=MultiRollResponseModel,
    responses={
        200: {
            "description": "Successful Response",
            "content": {
                "application/json": {
                    "example": {
                        "dice_sides": 6,
                        "results": [4, 2, 5],
                        "min_value": 2,
                        "max_value": 5,
                        "average": 3.67,
                        "median": 4,
                        "total": 11,
                    }
                }
            },
        }
    },
)
async def multi_roll_dice(sides: int = 6, rolls: int = 1) -> MultiRollResponseModel:
    """Roll a dice with a specified number of sides multiple times.

     Parameters
     ----------
     sides : int, optional
         The number of sides on the dice (default is 6).
     rolls : int, optional
         The number of times to roll the dice (default is 1).

     Returns
     -------
         A dictionaries containing the results of the dice rolls.

     Example
     -------
     ```
         GET /multi-roll?sides=6&rolls=3
         Response: {"dice_sides": 6,
         "results": [4, 2, 5],
         "min_value": 2,
         "max_value": 5,
         "average": 3.67,
         "median": 4,
         "total": 11
         }
    ```

    """

    dice = Dice(sides)
    results = [dice.roll() for _ in range(rolls)]
    return MultiRollResponseModel(
        dice_sides=sides,
        results=results,
        min_value=min(results),
        max_value=max(results),
        average=mean(results),
        median=median(results),
        total=sum(results),
    )
