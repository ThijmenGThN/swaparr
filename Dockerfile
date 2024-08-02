
# ----- Build Stage -----

# FROM rust:1-bookworm AS build

# ARG TARGETARCH

# ENV CROSS_CONTAINER_IN_CONTAINER=true
# ENV CROSS_CONTAINER_ENGINE=podman

# WORKDIR /swaparr

# COPY src ./src
# COPY Cargo* ./

# RUN apt update
# RUN apt install -y podman

# RUN cargo install cross
# RUN cross build --release --target $TARGET 

# RUN mv /swaparr/target/$TARGET/release/swaparr /opt

# ----- Runtime Stage -----

FROM scratch

# COPY --from=build /opt/swaparr /

ARG TARGETARCH
COPY ./target/$TARGETARCH/release/swaparr /

CMD ["/swaparr"]
