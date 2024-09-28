# Etapa de build
FROM rust:1.81.0-alpine as builder

# Instala dependências necessárias
RUN apk add --no-cache musl-dev openssl-dev build-base ffmpeg

WORKDIR /usr/src/app

# Copie o arquivo Cargo.toml e Cargo.lock
COPY Cargo.toml Cargo.lock ./

# Baixe as dependências do projeto
RUN cargo fetch

# Copie o restante do código do projeto
COPY . .

# Compile o projeto em modo release
RUN cargo build --release

# Etapa final
FROM alpine:latest

# Instale as dependências necessárias para executar o binário
RUN apk add --no-cache libgcc libstdc++ openssl ffmpeg

# Copie o binário compilado da etapa de construção
COPY --from=builder /usr/src/app/target/release/video-converter /usr/local/bin/video-converter

# Defina o ponto de entrada do contêiner
ENTRYPOINT ["video-converter"]

# Exponha a porta que a aplicação irá usar
EXPOSE 3000