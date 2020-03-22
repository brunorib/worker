FROM rust

WORKDIR /usr/src/myapp

COPY . .

RUN cargo build --release

RUN cargo install --path .

CMD ["/usr/local/cargo/bin/worker-mq"]