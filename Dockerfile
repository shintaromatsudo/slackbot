FROM rust as builder

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN mkdir src && touch /src/lib.rs
# RUN rustup component add rustfmt
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
# COPY ./.env ./.env
RUN cargo install --locked --path .

FROM debian:buster-slim
# RUN apt-get update && rm -rf /var/lib/apt/lists/*
RUN apt-get update && apt-get -y install libssl1.1 ca-certificates
COPY --from=builder /usr/local/cargo/bin/slackbot .
# COPY ./.env /usr/local/cargo/bin/slackbot/.env
CMD ["./slackbot"]
