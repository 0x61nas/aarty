FROM lukemathwalker/cargo-chef:0.1.63-rust-1.76-slim-buster AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --locked \
  && rm -f target/release/deps/aarty*

FROM debian:buster-slim as runner
COPY --from=builder /app/target/release/aarty /usr/local/bin
WORKDIR /app
ENTRYPOINT ["aarty"]
