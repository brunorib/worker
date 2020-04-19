FROM rust

WORKDIR /usr/src/myapp

COPY . .

RUN cargo build --release

RUN cargo install --path .

ENV RUST_LOG=info

CMD ["/usr/local/cargo/bin/worker-mq"]