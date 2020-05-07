FROM rust:1.43
WORKDIR /usr/code

COPY . .

RUN rustup component add rustfmt

RUN cargo install --path .
RUN cargo build --release

COPY --from=build /usr/code/target/release/routeguide-server .

EXPOSE 10000

CMD ["./routeguide-server"]
