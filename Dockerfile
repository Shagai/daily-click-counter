# syntax=docker/dockerfile:1
FROM rust:latest AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
ENV APP_DATA_PATH=/app/data/state.json
ENV PORT=8080
RUN mkdir -p /app/data

COPY --from=builder /app/target/release/web_app /usr/local/bin/web_app
EXPOSE 8080
CMD ["/usr/local/bin/web_app"]
