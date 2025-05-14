#!/bin/sh

mkdir -p sample.d

which usql || exec sh -c 'echo usql missing.; exit 1'

export ENV_ZIP_FILENAME=./sample.d/input.zip

export PGHOST=127.0.0.1
export PGUSER=postgres
export PGDATABASE=postgres
export PGPASSWORD="${PGPASSWORD}"

geninput(){
	echo creating input zip file...

	which rs-table2pgcopy || exec sh -c '
		echo rs-table2pgcopy missing.;
		echo install it using cargo: crates.io/crates/rs-table2pgcopy;
		exit 1
	'

	usql "pg://${PGUSER}@${PGHOST}" -c "
		CREATE TABLE IF NOT EXISTS tab1(
			id BIGSERIAL PRIMARY KEY,
			key TEXT NOT NULL,
			val TEXT NOT NULL
		)
	"
	
	usql "pg://${PGUSER}@${PGHOST}" -c "
		CREATE TABLE IF NOT EXISTS tab2(
			id BIGSERIAL PRIMARY KEY,
			key TEXT NOT NULL,
			val TEXT NOT NULL
		)
	"

	usql "pg://${PGUSER}@${PGHOST}" -c "
		INSERT INTO tab1(key, val)
		SELECT 'hello', 'world'
		WHERE 'world' <> COALESCE((
			SELECT val FROM tab1 WHERE id=1
		), '')
	"

	usql "pg://${PGUSER}@${PGHOST}" -c "
		INSERT INTO tab2(key, val)
		SELECT 'helo', 'wrld'
		WHERE 'wrld' <> COALESCE((
			SELECT val FROM tab2 WHERE id=1
		), '')
	"

	ENV_TABLE_NAME=tab1 rs-table2pgcopy > ./sample.d/tab1.pgcopy
	ENV_TABLE_NAME=tab2 rs-table2pgcopy > ./sample.d/tab2.pgcopy

	find sample.d/ -type f -name '*.pgcopy' |
		zip \
		-@ \
		-T \
		-v \
		-o \
		-j \
		"${ENV_ZIP_FILENAME}"
}

test -f "${ENV_ZIP_FILENAME}" || geninput

usql "pg://${PGUSER}@${PGHOST}" -c "TRUNCATE tab1"
usql "pg://${PGUSER}@${PGHOST}" -c "TRUNCATE tab2"

usql "pg://${PGUSER}@${PGHOST}" -c "SELECT COUNT(*) FROM tab1"
usql "pg://${PGUSER}@${PGHOST}" -c "SELECT COUNT(*) FROM tab2"

./rs-zip2pgcopy2tables

usql "pg://${PGUSER}@${PGHOST}" -c "SELECT COUNT(*) FROM tab1"
usql "pg://${PGUSER}@${PGHOST}" -c "SELECT COUNT(*) FROM tab2"
