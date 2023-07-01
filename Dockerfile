# syntax=docker/dockerfile:1.4

FROM rust:latest AS base-builder

RUN apt update && apt install -y --no-install-recommends \
	gettext-base \
	musl-tools \
	musl-dev \
	postgresql-client \
	&& rm -rf /var/lib/apt/lists/*
RUN update-ca-certificates
RUN rustup toolchain install nightly --profile minimal --component rustfmt
RUN rustup target add --toolchain nightly x86_64-unknown-linux-musl
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup component add clippy

# Create app user
ARG USER=user
ARG GROUP=user
ARG UID=1000
ARG GID=1000

RUN groupadd -g $GID $GROUP && useradd -u $UID -g $GROUP $USER

RUN mkdir /build /app /app/bin /app/data  && \
	chown user:user /build /app /app/bin /app/data

WORKDIR /build

USER $USER:$GROUP

FROM base-builder AS build-src

COPY .rustfmt.toml .rustfmt.toml
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

FROM build-src AS test

ARG PGHOST=localhost
ARG PGPORT=5432
ARG MARKET_PROFILE=test

ENV RUST_BACKTRACE=full
ENV SQLX_OFFLINE=true
ENV DATABASE_URL="postgres://market_app@${PGHOST}:${PGPORT}/market_db"

RUN cargo test --target=x86_64-unknown-linux-musl --verbose --release -p market

FROM build-src AS builder
RUN cargo build --release --target x86_64-unknown-linux-musl --package market --locked \
	--bin market && cp ./target/x86_64-unknown-linux-musl/release/market /app/bin/

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
