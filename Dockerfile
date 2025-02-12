FROM rust:latest

WORKDIR /app

COPY . /app

RUN cargo build --release

RUN ./target/release/gerencia_memoria_so
