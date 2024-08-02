
FROM scratch

ARG TARGET_PATH

COPY ./target/$TARGET_PATH/release/swaparr /

CMD ["/swaparr"]
