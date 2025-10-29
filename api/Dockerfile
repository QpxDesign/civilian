FROM rust:1.89.0 as builder

# PREP
RUN cargo install cargo-chef --version ^0.1
FROM base AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# BUILD
FROM base as builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
WORKDIR /app
ADD . /app
RUN --mount=type=cache,target=~/.cargo/registry/cache cargo build --release

# /home/quinn/.cargo/registry
# PROD
EXPOSE 1947
FROM gcr.io/distroless/cc
COPY --from=builder /app/assets /assets
COPY --from=builder /app/.env /.env
COPY --from=builder /app/target/release/civilian-api /

CMD ["./civilian-api"]
