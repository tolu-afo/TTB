# syntax=docker/dockerfile:1.7-labs
FROM rust:1.82.0

RUN cargo install diesel_cli
RUN git clone https://github.com/tolu-afo/TTB.git

WORKDIR /TTB

RUN cargo build --release

CMD ["sh", "./scripts/startup.sh"]
