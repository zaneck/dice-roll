import pydantic_settings


class Settings(pydantic_settings.BaseSettings):
    """Application settings."""

    debug: bool = True
    title: str = "Dice Roll API"
    description: str = "Dice rolling API - FAST API implementation"


settings = Settings()
