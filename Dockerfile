
# ----- Build Stage -----

FROM rust:1-bookworm AS build

ARG TARGETARCH
ENV CROSS_CONTAINER_IN_CONTAINER=true

WORKDIR /swaparr

COPY src ./src
COPY Cargo* ./

RUN cargo install cross

RUN case "$TARGETARCH" in \
    "amd64") TARGET="x86_64-unknown-linux-musl" ;; \
    "arm") TARGET="aarch64-unknown-linux-gnu" ;; \
    *) echo "Unsupported architecture: $TARGETARCH" && exit 1 ;; \
    esac && \
    rustup target add $TARGET && \
    cross cargo build --release --target $TARGET && \
    mv /swaparr/target/$TARGET/release/swaparr /opt


# ----- Runtime Stage -----

FROM scratch AS runtime

COPY --from=build /opt/swaparr /

CMD ["/swaparr"]
