
# ----- Build Stage -----

FROM rust:1-bookworm AS build

ARG TARGETARCH

WORKDIR /swaparr

COPY src ./src
COPY Cargo* ./

RUN date -s "$(wget -qSO- --max-redirect=0 google.com 2>&1 | grep Date: | cut -d' ' -f5-8)Z"
RUN apt update && apt install -y libssl-dev musl-tools

RUN case "$TARGETARCH" in \
    "amd64") TARGET="x86_64-unknown-linux-musl" ;; \
    "arm64") TARGET="aarch64-unknown-linux-gnu" ;; \
    esac && \
    rustup target add $TARGET && \
    cargo build --release --target $TARGET && \
    mv /swaparr/target/$TARGET/release/swaparr /opt

# ----- Runtime Stage -----

FROM scratch AS runtime

COPY --from=build /opt/swaparr /

CMD ["/swaparr"]
