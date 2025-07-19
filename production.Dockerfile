# syntax=docker/dockerfile:1.7-labs
FROM rust:1.88.0

RUN cargo install diesel_cli
ARG CACHE_DATE=not_a_date

RUN git clone https://github.com/tolu-afo/TTB.git

WORKDIR /TTB

RUN cargo build --release

CMD ["sh", "/TTB/scripts/startup.sh"]
