
# ----- Build Stage -----
FROM rust:1.76-bookworm AS build

WORKDIR /swaparr

COPY src ./src
COPY Cargo* ./

RUN apt update
RUN apt install -y libssl-dev musl-tools

RUN rustup target add x86_64-unknown-linux-musl

RUN echo "openssl = { version = \"0.10\", features = [\"vendored\"] }" >> Cargo.toml

RUN cargo build --release --target x86_64-unknown-linux-musl


# ----- Package Stage -----
FROM scratch

COPY --from=build /swaparr/target/x86_64-unknown-linux-musl/release/swaparr /swaparr

CMD ["/swaparr"]
