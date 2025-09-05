"""Routes usefull for api health check"""

from fastapi import APIRouter
from pydantic import BaseModel

router = APIRouter(tags=["Status"])


class StatusResponseModel(BaseModel):
    status: str = "OK"


@router.get("/status", response_model=StatusResponseModel)
async def status() -> StatusResponseModel:
    """Check status

    This function is used to check the status of the application.

    Returns
    -------
    ```
        {"status": "OK"}
    ```

    """
    return StatusResponseModel()
