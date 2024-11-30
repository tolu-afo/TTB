# syntax=docker/dockerfile:1.7-labs
FROM rust:1.82.0

RUN cargo install diesel_cli
ARG CACHE_DATE=not_a_date

WORKDIR /TTB

COPY . .

RUN cargo build --release

CMD ["sh", "/TTB/scripts/startup.sh"]
