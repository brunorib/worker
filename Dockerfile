FROM rust:slim

WORKDIR /usr/src/myapp

COPY . .

RUN cargo build --release

RUN cargo install --path .

EXPOSE 3550

CMD ["/usr/local/cargo/bin/worker-mq"]