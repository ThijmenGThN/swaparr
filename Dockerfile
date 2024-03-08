
# ----- Build Stage -----
FROM rust:1.76-bookworm AS build

WORKDIR /swaparr

COPY src ./src
COPY Cargo* ./

RUN cargo build --release


# ----- Package Stage -----
FROM debian:bookworm-slim

RUN apt update
RUN apt install -y libssl-dev
RUN apt clean

COPY --from=build /swaparr/target/release/swaparr /usr/local/bin/swaparr

CMD ["sh", "-c", "swaparr $BASEURL $APIKEY $PLATFORM $TIME_THRESHOLD $SIZE_THRESHOLD $CHECK_INTERVAL $STRIKE_THRESHOLD $AGGRESSIVE_STRIKES"]
