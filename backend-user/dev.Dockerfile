ARG RUST_VERSION=1.90.0

FROM rust:${RUST_VERSION}-slim

WORKDIR /backend-user

COPY Cargo.toml Cargo.lock ./

RUN cargo fetch

COPY . .

RUN cargo install --locked cargo-watch \
                            cargo-audit \
                            cargo-deny \
                            cargo-tarpaulin \
                            cargo-nextest

RUN rustup component add clippy rustfmt

CMD ["cargo", "watch", "-x", "run"]
