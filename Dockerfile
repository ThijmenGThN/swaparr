
FROM alpine:3.12

COPY ./swaparr /usr/local/bin

CMD ["swaparr"]
