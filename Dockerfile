# Etapa de build
FROM rust:1.81.0-slim-bullseye as builder

# Instala dependências necessárias
RUN apt-get update && apt-get install -y --no-install-recommends \
  musl-dev \
  libssl-dev \
  build-essential \
  ffmpeg \
  pkg-config \
  && rm -rf /var/lib/apt/lists/*

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
FROM debian:bullseye-slim

# Instale as dependências necessárias para executar o binário
RUN apt-get update && apt-get install -y --no-install-recommends \
  libgcc-s1 \
  libstdc++6 \
  libssl1.1 \
  ffmpeg \
  && rm -rf /var/lib/apt/lists/*

# Copie o binário compilado da etapa de construção
COPY --from=builder /usr/src/app/target/release/video-converter /usr/local/bin/video-converter

# Defina o ponto de entrada do contêiner
ENTRYPOINT ["video-converter"]

# Exponha a porta que a aplicação irá usar
EXPOSE 8080
