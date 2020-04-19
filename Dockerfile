FROM rust

WORKDIR /usr/src/myapp

COPY . .

RUN cargo build --release

RUN cargo install --path .

CMD ["RUST_LOG=info /usr/local/cargo/bin/worker-mq"]