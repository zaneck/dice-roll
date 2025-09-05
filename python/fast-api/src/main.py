from fastapi import FastAPI

from config import Settings, settings
from rest.routers import roll, status


def bootstrap(api_settings: Settings) -> FastAPI:
    """Start API and services."""
    api = FastAPI(
        debug=api_settings.debug,
        title=api_settings.title,
        description=api_settings.description,
    )

    api.include_router(status.router)
    api.include_router(roll.router)

    return api


app: FastAPI = bootstrap(settings)
