# syntax=docker/dockerfile:1.4

FROM rust:1.70-slim-buster as base-builder

RUN rustup component add clippy

# Create app user
ARG USER=user
ARG GROUP=user
ARG UID=1000
ARG GID=1000

RUN groupadd -g $GID $GROUP && useradd -u $UID -g $GROUP $USER

RUN mkdir /build /app /app/bin /app/data  && \
    chown user:user /build /app /app/bin /app/data

RUN apt-get update -y && \
  apt-get install -y pkg-config make g++ libssl-dev && \
  rustup target add x86_64-unknown-linux-gnu

WORKDIR /build

USER $USER:$GROUP

FROM base-builder AS build-src

COPY .rustfmt.toml .rustfmt.toml
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY market ./market

FROM build-src AS test

ARG PGHOST=localhost
ARG PGPORT=5432
ARG MARKET_PROFILE=test

ENV RUST_BACKTRACE=full
ENV SQLX_OFFLINE=true
ENV DATABASE_URL="postgres://market_app@${PGHOST}:${PGPORT}/market_db"

RUN cargo test --verbose --release -p market

FROM build-src AS builder
RUN cargo build --release && cp ./target/release/market /app/bin/

FROM scratch AS market

WORKDIR /app

# Use an unprivileged user.
USER user:user

EXPOSE 8000

CMD ["/app/bin/market"]

# Import from builder.
COPY --from=base-builder /etc/passwd /etc/passwd
COPY --from=base-builder /etc/group /etc/group

# Copy our build
COPY --from=builder /app/ ./

