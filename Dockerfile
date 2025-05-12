FROM rust:alpine as build
RUN apk update && apk add alpine-sdk libressl-dev
COPY ./ /attos_pixoo64
WORKDIR /attos_pixoo64
RUN cargo build --release

FROM alpine:latest
RUN apk update && apk add tzdata && \
  cp /usr/share/zoneinfo/Asia/Tokyo /etc/localtime && \
  echo "Asia/Tokyo" > /etc/timezone && \
  apk del tzdata
COPY --from=build /attos_pixoo64/target/release/attos_pixoo64 /
CMD ["./attos_pixoo64"]
