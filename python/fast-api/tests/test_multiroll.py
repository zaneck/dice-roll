from fastapi.testclient import TestClient


def test_multiroll_no_queryparams(fake_app: TestClient) -> None:
    response = fake_app.get("/multi-roll")
    assert response.status_code == 200

    assert response.json()["dice_sides"] == 6
    assert len(response.json()["results"]) == 1
    assert 1 <= response.json()["results"][0] <= 6


def test_multiroll_with_queryparams_sides(fake_app: TestClient) -> None:
    response = fake_app.get("/multi-roll?sides=20")
    assert response.status_code == 200

    assert response.json()["dice_sides"] == 20
    assert len(response.json()["results"]) == 1
    assert 1 <= response.json()["results"][0] <= 20


def test_multiroll_with_queryparams_sides_and_rolls(fake_app: TestClient) -> None:
    response = fake_app.get("/multi-roll?sides=10&rolls=5")
    assert response.status_code == 200

    assert response.json()["dice_sides"] == 10
    assert len(response.json()["results"]) == 5
    for result in response.json()["results"]:
        assert 1 <= result <= 10
