# BUILD
FROM rust:1.89.0 as builder
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
