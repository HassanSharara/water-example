FROM rust:latest as builder
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo run --release
EXPOSE 8084
CMD ["./target/release/water"]
