# Build stage
FROM rust:1.75-buster as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:buster-slim
WORKDIR /usr/local/bin
COPY --from=builder /app/target/release/yorehub .
EXPOSE 8080
ENTRYPOINT ["./yorehub"]
