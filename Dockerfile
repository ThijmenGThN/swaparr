
# ----- Build Stage -----

FROM rust:1-bookworm AS build

WORKDIR /swaparr

COPY src ./src
COPY Cargo* ./

# Accept ARCH as env variable.
ARG ARCH
ENV ARCH=${ARCH:-x86_64}

# Add OpenSSL dependency.
RUN echo "openssl = { version = \"0.10\", features = [\"vendored\"] }" >> Cargo.toml

# Install build tools.
RUN apt update && apt install -y libssl-dev musl-tools
RUN if [ "$ARCH" = "arm64" ]; then apt install -y gcc-aarch64-linux-gnu g++-aarch64-linux-gnu

# Add musl target.
RUN if [ "$ARCH" = "amd64" ]; then rustup target add x86_64-unknown-linux-musl; \
    elif [ "$ARCH" = "arm64" ]; then rustup target add aarch64-unknown-linux-musl; \
    else echo "Unsupported architecture: $ARCH"; exit 1; fi

# Build Swaparr.
RUN if [ "$ARCH" = "amd64" ]; then cargo build --release --target x86_64-unknown-linux-musl; \
    elif [ "$ARCH" = "arm64" ]; then cargo build --release --target aarch64-unknown-linux-musl; \
    else echo "Unsupported architecture: $ARCH"; exit 1; fi



# ----- Package Stage -----

FROM scratch

# Accept ARCH as env variable.
ARG ARCH
ENV ARCH=${ARCH:-x86_64}

# Import Swaparr binary.
COPY --from=build /swaparr/target/${ARCH}-unknown-linux-musl/release/swaparr /swaparr

CMD ["/swaparr"]
