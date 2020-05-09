# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------
FROM rust:1.43 as cargo-build

WORKDIR /code

COPY . .

RUN rustup component add rustfmt
RUN cargo build --release

# # ------------------------------------------------------------------------------
# # Final Stage
# # ------------------------------------------------------------------------------
FROM alpine:3.11.6

WORKDIR /routerust

EXPOSE 8000
COPY --from=cargo-build /code/target/release/routeguide* /routerust

CMD ["./routeguide-server"]

