
# ----- Build Stage -----

FROM rust:1-bookworm AS build

WORKDIR /swaparr

COPY src ./src
COPY Cargo* ./

RUN cargo install --path .

# ----- Runtime Stage -----

FROM scratch AS runtime

COPY --from=build /usr/local/cargo/bin/swaparr /swaparr

CMD ["/swaparr"]
