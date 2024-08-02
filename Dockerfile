
# ----- Build Stage -----

FROM rust:1-bookworm AS build

ARG TARGETARCH

WORKDIR /swaparr

COPY src ./src
COPY Cargo* ./

RUN apt update && apt install -y libssl-dev musl-tools

RUN case "$TARGETARCH" in \
    "amd64") ;; \
    "arm") RUN apt install -y aarch64-linux-gnu-gcc ;; \
    *) echo "Unsupported architecture: $TARGETARCH" && exit 1 ;; \
    esac

RUN case "$TARGETARCH" in \
    "amd64") TARGET="x86_64-unknown-linux-musl" ;; \
    "arm") TARGET="aarch64-unknown-linux-gnu" ;; \
    *) echo "Unsupported architecture: $TARGETARCH" && exit 1 ;; \
    esac && \
    rustup target add $TARGET && \
    cargo build --release --target $TARGET && \
    mv /swaparr/target/$TARGET/release/swaparr /opt

# ----- Runtime Stage -----

FROM scratch AS runtime

COPY --from=build /opt/swaparr /

CMD ["/swaparr"]
