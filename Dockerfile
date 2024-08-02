
# ----- Build Stage -----

FROM rust:1-bookworm AS build

ARG TARGETARCH

WORKDIR /usr/swaparr

COPY src ./src
COPY Cargo* ./

RUN apt update 
RUN apt install -y libssl-dev musl-tools docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

RUN cargo install cross

RUN case "$TARGETARCH" in \
    "amd64") TARGET="x86_64-unknown-linux-musl" ;; \
    "arm") TARGET="aarch64-unknown-linux-musl" ;; \
    *) echo "Unsupported architecture: $TARGETARCH" && exit 1 ;; \
    esac && \
    cross build --release --target $TARGET && \
    mv /usr/swaparr/target/$TARGET/release/swaparr /opt

# ----- Runtime Stage -----

FROM scratch AS runtime

COPY --from=build /opt/swaparr /

CMD ["/swaparr"]
