FROM ubuntu:latest AS builder
RUN apt-get update && \
    apt-get install -y cmake gcc curl pkg-config libssl-dev git &&\
    rm -rf  /var/lib/apt/lists/*
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /app
RUN cargo init --bin  # 创建临时项目用于下载依赖
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release # 下载依赖并预构建
COPY . .
RUN cargo build --release

FROM ubuntu:latest
#RUN apt-get update && \
#    apt-get install -y yt-dlp &&\
#    rm -rf /var/lib/apt/lists/*
RUN apt-get update &&\
    apt-get install -y curl python3 &&\
    curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -o /usr/local/bin/yt-dlp &&\
    chmod a+rx /usr/local/bin/yt-dlp
# 直接使用yt-dlp镜像
WORKDIR /app
COPY --from=builder /app/target/release/cc-bot /app/
CMD ["/app/cc-bot"]