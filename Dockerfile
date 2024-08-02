
FROM scratch

ARG TARGET_PATH

COPY ./swaparr /

CMD ["/swaparr"]
