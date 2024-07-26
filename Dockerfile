
# ----- Build Stage -----

FROM rust:1-bookworm AS build

WORKDIR /swaparr

COPY src ./src
COPY Cargo* ./

RUN cargo build --release --bin swaparr

# ----- Runtime Stage -----

FROM scratch AS runtime

COPY --from=build /swaparr/target/release/swaparr /swaparr

CMD ["/swaparr"]
