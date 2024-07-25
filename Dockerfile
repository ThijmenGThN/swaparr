
# ----- Build Stage -----
FROM rust:1-bookworm AS build

WORKDIR /swaparr

COPY src ./src
COPY Cargo* ./

# Add OpenSSL dependency to cargo. (Required in musl build.)
RUN echo "openssl = { version = \"0.10\", features = [\"vendored\"] }" >> Cargo.toml

# Install buildtools.
RUN apt update
RUN apt install -y libssl-dev musl-tools

# Add musl target.
RUN rustup target add $ARCH-unknown-linux-musl

# Build Swaparr.
RUN cargo build --release --target $ARCH-unknown-linux-musl


# ----- Package Stage -----
FROM scratch

# Copy Swaparr binary to scratch image.
COPY --from=build /swaparr/target/$ARCH-unknown-linux-musl/release/swaparr /swaparr

CMD ["/swaparr"]
