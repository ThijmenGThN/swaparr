
# ----- Build Stage -----

FROM rust:1-bookworm AS build

WORKDIR /swaparr

COPY src ./src
COPY Cargo* ./

RUN cargo install --path .

# ----- Package Stage -----

FROM scratch

COPY --from=build /usr/local/cargo/bin/swaparr /swaparr

CMD ["/swaparr"]
