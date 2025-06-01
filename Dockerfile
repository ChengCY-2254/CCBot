FROM ubuntu:latest AS builder
WORKDIR /app
COPY . .
RUN apt-get update && \
    apt-get install -y cmake gcc curl pkg-config libssl-dev && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo build --release

FROM ubuntu:latest
WORKDIR /app
RUN apt-get update && \
    apt-get install -y yt-dlp &&\
    rm  -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/cc-bot /app/
CMD ["/app/cc-bot"]