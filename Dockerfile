FROM lukemathwalker/cargo-chef:0.1.65-rust-1.76-alpine AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN sed -i '/^\[\[bench\]\]/,/^$/d' Cargo.toml
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
RUN cargo chef cook --release --recipe-path recipe.json
COPY Cargo.toml .
COPY Cargo.lock .
RUN sed -i '/^\[\[bench\]\]/,/^$/d' Cargo.toml
COPY src/ src/
RUN cargo build --release --locked \
  && rm -f target/release/deps/aarty*

FROM alpine:latest as runner
COPY --from=builder /app/target/release/aarty /usr/local/bin
WORKDIR /app
ENTRYPOINT ["aarty"]
