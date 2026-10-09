FROM --platform=linux/arm64 rust:1.96-bullseye
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential cmake pkg-config python3 patch git ca-certificates \
    libasound2-dev libudev-dev libsdl2-dev libegl1-mesa-dev libgles2-mesa-dev \
    libdrm-dev libgbm-dev zlib1g-dev libpng-dev libjpeg-dev libfreetype6-dev \
    libfontconfig1-dev libcurl4-openssl-dev libssl-dev libbz2-dev liblzma-dev \
    squashfs-tools unzip zip yasm nasm && rm -rf /var/lib/apt/lists/*
ENV CARGO_BUILD_JOBS=4
WORKDIR /repo
