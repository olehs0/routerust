FROM rust:1.43
WORKDIR /usr/code

COPY . .

RUN rustup component add rustfmt

RUN cargo install --path .
RUN cargo build --release

EXPOSE 10000

CMD ["./target/release/routeguide-server"]
