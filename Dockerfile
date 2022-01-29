FROM ubuntu:latest AS idie-deps

WORKDIR /app

# Git & utils
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends --fix-missing -y tzdata git curl ca-certificates

## Rust
RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y  --default-toolchain nightly

ENV PATH="/root/.cargo/bin:${PATH}"

## Youki
RUN apt-get install -y \
      pkg-config \
      libsystemd-dev \
      libdbus-glib-1-dev \
      build-essential \
      libelf-dev \
      libseccomp-dev

RUN git clone https://github.com/containers/youki.git \
  && cd youki \
  && ./build.sh

FROM rust:1.55.0 AS idie-dev
RUN mkdir -p /app
WORKDIR /app

RUN apt-get update && apt-get install -y \
      wget \
      pkg-config \
      libsystemd-dev \
      libdbus-glib-1-dev \
      build-essential \
      libelf-dev \
      libseccomp-dev

ENV DOCKERIZE_VERSION v0.6.1
RUN wget https://github.com/jwilder/dockerize/releases/download/$DOCKERIZE_VERSION/dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
  && tar -C /usr/local/bin -xzvf dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
  && rm dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz

COPY . .

RUN cargo install cargo-watch

COPY --from=idie-deps /app/youki/youki /deps/youki
