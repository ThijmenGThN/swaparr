
# ----- Build Stage -----

FROM rust:1-bookworm AS build

ARG TARGETARCH

WORKDIR /swaparr

COPY src ./src
COPY Cargo* ./

RUN apt update && apt install -y libssl-dev musl-tools

RUN case "$TARGETARCH" in \
    "linux/amd64") TARGET="x86_64-unknown-linux-musl" ;; \
    "linux/arm/v7") TARGET="aarch64-unknown-linux-gnu" ;; \
    esac && \
    rustup target add $TARGET && \
    cargo build --release --target $TARGET && \
    mv /swaparr/target/$TARGET/release/swaparr /opt

# ----- Runtime Stage -----

FROM scratch AS runtime

COPY --from=build /opt/swaparr /

CMD ["/swaparr"]
