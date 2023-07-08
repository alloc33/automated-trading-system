# vim: set ft=make
# code: language=makefile

default:
    just --list

export RUST_LOG := env_var_or_default("RUST_LOG", "debug,sqlx=error")
export DATABASE_URL := "postgres://market_app@localhost:5432/market_db"

# run development server
runserver:
    # ignore files that sqlx prepare touches during offline query data preparation
    cargo watch -x "run -p market" -w market/src --why --ignore market/src/lib.rs --ignore market/src/main.rs

check:
    cargo check

##################################################
################### DATABASE #####################
##################################################

db-start:
    docker-compose up -d pgdb

# start with a clean database
db-fresh: && migrate
    docker-compose down
    just db-start
    sleep 2

# run `cargo sqlx migrate` subcommand (`run` by default)
migrate subcommand="run":
    cargo sqlx migrate {{ subcommand }}  --source=./market/migrations

# generate market/sqlx-data.json for offline mode
for-offline: db-start migrate
    cd market && cargo sqlx prepare --merged -- --lib --tests

# enter the PostgreSQL database shell
db-shell user="market_app" db="market_db":
    pgdb psql -d {{ db }} -U {{ user }}

##################################################
##################### TEST #######################
##################################################

alias t := test

# run all package tests (market by default)
test test_name="" package="market":
    cargo test -p {{ package }} --color always {{ test_name }} --

