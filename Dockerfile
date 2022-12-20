FROM rust as builder

RUN cargo install sea-orm-cli
COPY ./Cargo.* ./
COPY ./src ./src
COPY ./entity ./entity
COPY ./migration ./migration
COPY ./api ./api
RUN cargo install --locked --path .
RUN cargo build --release

# FROM rust
# RUN apt-get update && apt-get -y install libssl1.1 ca-certificates
# COPY --from=builder /usr/local/cargo/bin/slackbot .
COPY ./.env ./.env
CMD ["./target/release/slackbot"]
