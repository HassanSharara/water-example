FROM rust:latest as builder

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo run --release

CMD ["./target/release/water"]
