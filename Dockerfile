FROM messense/rust-musl-cross:x86_64-musl as chef

# remove the line below when switching to >=rust:1.70.0. sparse mode is planned to be the default in Rust 1.70.0
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
ENV SQLX_OFFLINE=true

RUN cargo install cargo-chef
WORKDIR /market

FROM chef AS planner
COPY . .
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
