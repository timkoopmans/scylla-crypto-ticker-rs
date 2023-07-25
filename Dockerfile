FROM rust:1.68.2 as chef
WORKDIR app
RUN cargo install cargo-chef

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin scylla-crypto-ticker

FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/scylla-crypto-ticker /usr/bin/scylla-crypto-ticker

CMD ["scylla-crypto-ticker"]