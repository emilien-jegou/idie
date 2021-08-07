FROM rust

RUN mkdir -p /app
WORKDIR /app

COPY . /app

RUN cargo install cargo-watch

# Prerun cargo build to avoid doing it when running the container
RUN cargo build

CMD cargo watch -x run
