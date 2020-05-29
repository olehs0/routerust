# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------
FROM rust:1.43 as cargo-build

WORKDIR /code

COPY . .

RUN rustup component add rustfmt
RUN cargo build --release

# ------------------------------------------------------------------------------
# Cargo Deploy Stage
# ------------------------------------------------------------------------------
FROM alpine
EXPOSE 8000

COPY --from=cargo-build /code/target/release/* ./
RUN apk --no-cache add ca-certificates
WORKDIR /root/

CMD ["./routeguide-server"]

