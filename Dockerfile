FROM rust:1.78-alpine as builder

WORKDIR /usr/src/pinecil2mqtt
COPY . .

RUN apk add --no-cache dbus-dev musl-dev
RUN cargo install --path .

FROM alpine:3.19
LABEL org.opencontainers.image.source=https://github.com/excieve/pinecil2mqtt

COPY --from=builder /usr/local/cargo/bin/pinecil2mqtt /usr/local/bin/pinecil2mqtt

RUN apk add --no-cache dbus-libs

ENTRYPOINT ["pinecil2mqtt"]
