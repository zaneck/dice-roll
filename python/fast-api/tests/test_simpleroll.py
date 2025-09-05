from fastapi.testclient import TestClient


def test_roll_no_queryparams(fake_app: TestClient) -> None:
    response = fake_app.get("/roll")
    assert response.status_code == 200

    assert response.json()["dice_sides"] == 6
    assert 1 <= response.json()["result"] <= 6


def test_roll_with_queryparams(fake_app: TestClient) -> None:
    response = fake_app.get("/roll?sides=20")
    assert response.status_code == 200

    assert response.json()["dice_sides"] == 20
    assert 1 <= response.json()["result"] <= 20
