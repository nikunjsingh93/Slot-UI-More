FROM rust:1.96-bullseye
RUN sed -i 's|deb.debian.org/debian-security|archive.debian.org/debian-security|g' /etc/apt/sources.list \
    && apt-get -o Acquire::Check-Valid-Until=false update \
    && apt-get install -y --no-install-recommends build-essential pkg-config \
       python3 patch git ca-certificates libasound2-dev libudev-dev \
       libegl1-mesa-dev libgles2-mesa-dev libdrm-dev libgbm-dev \
       e2fsprogs gdisk squashfs-tools unzip zip \
    && rm -rf /var/lib/apt/lists/*
ENV CARGO_BUILD_JOBS=4
WORKDIR /repo
