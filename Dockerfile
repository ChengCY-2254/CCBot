FROM rust AS builder
WORKDIR /app
COPY . .
RUN apt-get update &&  \
    apt-get install -y cmake gcc
RUN cargo build --release

#FROM alpine:3.21
#WORKDIR /app
#RUN apk update && \
#    apk add --no-cache yt-dlp gcompat &&\
#    mkdir config
FROM ubuntu:latest
WORKDIR /app
RUN apt-get update && \
    apt-get install -y yt-dlp
COPY --from=builder /app/target/release/cc-bot /app/
CMD ["/app/cc-bot"]