# ----- Build Stage -----
    FROM rust:1-bookworm AS build

    WORKDIR /swaparr
    
    COPY src ./src
    COPY Cargo* ./
    
    # Accept ARCH as a build argument and set a default value.
    ARG ARCH
    ENV ARCH=${ARCH:-x86_64}
    
    # Add OpenSSL dependency to cargo. (Required in musl build.)
    RUN echo "openssl = { version = \"0.10\", features = [\"vendored\"] }" >> Cargo.toml
    
    # Install build tools.
    RUN apt update && apt install -y libssl-dev musl-tools
    
    # Add musl target based on ARCH.
    RUN if [ "$ARCH" = "amd64" ]; then \
            rustup target add x86_64-unknown-linux-musl; \
        elif [ "$ARCH" = "arm64" ]; then \
            rustup target add aarch64-unknown-linux-musl; \
        else \
            echo "Unsupported architecture: $ARCH"; exit 1; \
        fi
    
    # Build Swaparr.
    RUN if [ "$ARCH" = "amd64" ]; then \
            cargo build --release --target x86_64-unknown-linux-musl; \
        elif [ "$ARCH" = "arm64" ]; then \
            cargo build --release --target aarch64-unknown-linux-musl; \
        fi
    
    
    # ----- Package Stage -----
    FROM scratch
    
    # Accept ARCH as a build argument and set a default value.
    ARG ARCH
    ENV ARCH=${ARCH:-x86_64}
    
    # Copy Swaparr binary to scratch image.
    COPY --from=build /swaparr/target/${ARCH}-unknown-linux-musl/release/swaparr /swaparr
    
    CMD ["/swaparr"]
    