run:
	python3 -m poetry run fastapi dev src/main.py

format:
	python3 -m ruff format src tests
	python3 -m ruff check --fix src tests

tests:
	python3 -m poetry run pytest -vvv

poetry_update:
	python3 -m poetry update

clean:
	rm -rf .pytest_cache
	find . -type d -name '__pycache__' -exec rm -rf {} +
	rm -rf .ruff_cache
	rm -rf .coverage
	rm -rf coverage-report


.PHONY: run format tests poetry_update clean