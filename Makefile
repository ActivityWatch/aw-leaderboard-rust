.PHONY: typecheck lint run test format format-check

SRCDIR = src/aw_leaderboard
DBDIR = /tmp/aw-leaderboard-db
DBCONTAINER = aw-leaderboard-postgres

typecheck:
	poetry run mypy ${SRCDIR}

lint:
	poetry run ruff ${SRCDIR}

run:
	poetry run uvicorn aw_leaderboard:app --reload

test:
	poetry run pytest tests/

format:
	poetry run black ${SRCDIR} tests

format-check:
	poetry run black --check ${SRCDIR} tests

db-run:
	docker container stop ${DBCONTAINER} || true
	docker container rm ${DBCONTAINER}
	docker run --name aw-leaderboard-postgres -e POSTGRES_USERNAME=user -e POSTGRES_PASSWORD=password -p 5432:5432 -v ${DBDIR}:/var/lib/postgresql/data -d postgres
