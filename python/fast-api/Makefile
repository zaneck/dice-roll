run:
	python3 -m poetry run fastapi dev src/main.py

format:
	python3 -m ruff format src tests
	python3 -m ruff check --fix src tests

tests:
	python3 -m poetry run pytest -vv

poetry_update:
	python3 -m poetry update

.PHONY: run format tests poetry_update