FROM rust:latest AS builder

# Instala o target MUSL para compilação estática
RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/app
COPY . .

# Compila para MUSL
RUN cargo build --release --target x86_64-unknown-linux-musl

# Usa uma imagem mínima Alpine
FROM alpine:latest

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/aula_pipeline /usr/local/bin/

CMD ["aula_pipeline"]