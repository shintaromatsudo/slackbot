FROM rust as builder

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN mkdir src && touch /src/lib.rs
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
RUN cargo install --locked --path .

FROM debian:buster-slim
RUN apt-get update && apt-get -y install libssl1.1 ca-certificates
COPY --from=builder /usr/local/cargo/bin/slackbot .
COPY ./.env ./.env
CMD ["./slackbot"]
