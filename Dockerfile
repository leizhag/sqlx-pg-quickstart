FROM rust as builder
WORKDIR /usr/src/sqlx-pg-quickstart
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/sqlx-pg-quickstart /usr/local/bin/sqlx-pg-quickstart
CMD ["sqlx-pg-quickstart"]
