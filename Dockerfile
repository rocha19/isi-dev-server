# Etapa de build
FROM rust:1.86 as builder

WORKDIR /app
COPY . .

# Instale dependências necessárias para compilar com OpenSSL embutido
RUN apt-get update && apt-get install -y pkg-config libssl-dev clang

# Configure ambiente para linkagem estática do OpenSSL
ENV OPENSSL_STATIC=1
ENV OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu
ENV OPENSSL_INCLUDE_DIR=/usr/include

# Compile o binário
RUN cargo build --release

# Etapa final com imagem mínima (sem dependências do sistema)
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y bash curl

WORKDIR /usr/local/bin

# Copia o binário do builder
COPY --from=builder /app/target/release/isi-dev .

# Copia schema.sql do contexto local (não do builder!)
COPY schema.sql .

CMD ["sh", "-c", "echo 'Starting application...'; sleep 2; ./isi-dev"]
