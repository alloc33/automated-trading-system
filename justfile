default:
    just --list

export RUST_LOG := env_var_or_default("RUST_LOG", "debug,sqlx=error")
export DATABASE_URL := "postgres://market_app@localhost:5432/market_db"

# run development server
runserver:
    # ignore files that sqlx prepare touches during offline query data preparation
    cargo watch -x "run -p market" # -w broker/src --why --ignore broker/src/lib.rs --ignore broker/src/main.rs

check:
    cargo check

##################################################
################### DATABASE #####################
##################################################

# enter the PostgreSQL database shell
db-shell user="market_app" db="market_db":
    docker-compose exec pgdb psql -d {{ db }} -U {{ user }}

##################################################
##################### TEST #######################
##################################################

alias t := test

# run all package tests (market by default)
test test_name="" package="market":
    cargo test -p {{ package }} --color always {{ test_name }} --

