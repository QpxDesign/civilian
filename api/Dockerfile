FROM rust:1.89.0 as base
RUN cargo install sccache --version ^0.7
RUN cargo install cargo-chef --version ^0.1
ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache

# PREP
FROM base AS planner
WORKDIR /app
COPY . .
RUN --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \ cargo chef prepare --recipe-path recipe.json

# BUILD
FROM base as builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \ cargo chef cook --release --recipe-path recipe.json
WORKDIR /app
ADD . /app
RUN --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \ cargo build --release

# PROD
EXPOSE 1947
FROM gcr.io/distroless/cc
COPY --from=builder /app/assets /assets
COPY --from=builder /app/.env /.env
COPY --from=builder /app/target/release/civilian-api /

CMD ["./civilian-api"]
