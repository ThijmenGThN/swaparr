
FROM alpine:3.12

COPY ./swaparr /usr/local/bin

HEALTHCHECK --interval=60s --timeout=5s --retries=3 \
  CMD cat /tmp/swaparr.health | grep -q "1" || exit 1

CMD ["swaparr"]
