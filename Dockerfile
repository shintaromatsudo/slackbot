FROM rust as builder

COPY ./Cargo.* ./
COPY ./src ./src
COPY ./api ./api
RUN cargo install --locked --path .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get -y install libssl1.1 ca-certificates
COPY --from=builder /usr/local/cargo/bin/slackbot .
COPY ./.env ./.env
CMD ["./slackbot"]
