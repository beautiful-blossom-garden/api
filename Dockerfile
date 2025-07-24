FROM rust:1.88-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y llvm llvm-dev libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

COPY . .

RUN export GYP_DEFINES="linux_use_gold_flags=0"

RUN cargo build --release

FROM debian:12-slim

WORKDIR /app

RUN apt-get update && apt-get install -y libpq-dev

COPY --from=builder /usr/local/cargo/bin/diesel ./diesel
COPY --from=builder /app/target/release/api ./
COPY migrations ./migrations
COPY entrypoint.sh ./

EXPOSE 3000

CMD ["sh", "entrypoint.sh"]