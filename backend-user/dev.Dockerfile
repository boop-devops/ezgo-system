ARG RUST_VERSION=1.90.0

FROM rust:${RUST_VERSION}

WORKDIR /backend-user

COPY . .

RUN cargo install cargo-watch --locked

CMD ["cargo", "watch", "-x", "run"]
