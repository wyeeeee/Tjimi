# 使用 Node.js 镜像作为前端构建环境
FROM node:lts-slim AS frontend-builder
WORKDIR /usr/src/app/frontend
# 安装pnpm
RUN npm install -g pnpm
# 复制前端源码
COPY frontend/ .
# 安装依赖并构建前端
RUN pnpm install && pnpm run build
# 注意：前端构建结果会输出到 ../static 目录中

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS backend-builder 
RUN apt-get update && apt-get install -y --no-install-recommends \
    cmake \
    clang \
    && rm -rf /var/lib/apt/lists/*
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
ENV RUSTFLAGS="-Awarnings --cfg tokio_unstable"
COPY --from=frontend-builder /usr/src/app/static ./static
RUN cargo build --release --bin clewdr --features no_fs

# 使用更小的基础镜像
FROM debian:bookworm-slim
WORKDIR /app
# 从后端构建阶段复制编译好的二进制文件
COPY --from=backend-builder /app/target/release/clewdr .
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 配置环境变量
ENV CLEWDR_IP=0.0.0.0
ENV CLEWDR_PORT=8484
ENV CLEWDR_CHECK_UPDATE=FALSE
ENV CLEWDR_AUTO_UPDATE=FALSE
ENV CLEWDR_TOKIO_CONSOLE=FALSE

# 暴露端口
EXPOSE 8484

# 启动命令
CMD ["./clewdr"]
