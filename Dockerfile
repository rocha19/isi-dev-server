FROM rust:1.79.0-buster as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /usr/local/bin
COPY --from=builder /app/target/release/isi-dev .
EXPOSE 3000
ENTRYPOINT ["./isi-dev"]
