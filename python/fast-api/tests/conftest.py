import pytest
from fastapi.testclient import TestClient

from main import app


@pytest.fixture
def fake_app() -> TestClient:
    """Build test app."""
    return TestClient(app)
