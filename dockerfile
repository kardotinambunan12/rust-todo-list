
FROM rust:1.86.0 as builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY . .
RUN cargo build --release


FROM rust:1.86.0-slim

RUN useradd -m appuser
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/todo-list /usr/local/bin/app

USER appuser
EXPOSE 3000

CMD ["/usr/local/bin/app"]
