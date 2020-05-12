# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------
FROM rust:1.43 as cargo-build

WORKDIR /code

COPY . .

RUN rustup component add rustfmt
RUN cargo build --release
EXPOSE 8000

CMD ["./target/release/routeguide-server"]

