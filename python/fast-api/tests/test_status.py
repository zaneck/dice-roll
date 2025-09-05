from fastapi.testclient import TestClient


def test_status_code(fake_app: TestClient) -> None:
    response = fake_app.get("/status")
    assert response.status_code == 200
    assert response.json() == {"status": "OK"}
