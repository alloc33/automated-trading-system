FROM messense/rust-musl-cross:x86_64-musl as chef

# Create app user
ARG USER=user
ARG GROUP=user
ARG UID=1000
ARG GID=1000

RUN groupadd -g $GID $GROUP && useradd -u $UID -g $GROUP $USER

RUN cargo install cargo-chef
WORKDIR /market

FROM chef AS planner
COPY . .

ARG PGHOST=localhost
ARG PGPORT=5432
ARG MARKET_PROFILE=test

ENV RUST_BACKTRACE=full
ENV SQLX_OFFLINE=true
ENV DATABASE_URL="postgres://market_app@${PGHOST}:${PGPORT}/market_db"

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /market/recipe.json recipe.json
RUN rustup target add x86_64-unknown-linux-musl
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# We do not need the Rust toolchain to run the binary!
FROM scratch
COPY --from=builder /market/target/x86_64-unknown-linux-musl/release/market /usr/local/bin/market
EXPOSE 8000
ENTRYPOINT [ "market" ]
