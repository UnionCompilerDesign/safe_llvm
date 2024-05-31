FROM ubuntu:20.04

ENV DEBIAN_FRONTEND=noninteractive

WORKDIR /usr/src/safe_llvm

RUN apt-get update && \
    apt-get install -y \
    ca-certificates \
    wget \
    curl \
    gnupg \
    software-properties-common \
    build-essential \
    cmake \
    libfreetype6-dev \
    libfontconfig1-dev \
    xclip \
    valgrind \
    libzstd-dev && \
    wget -qO - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add - && \
    apt-add-repository "deb http://apt.llvm.org/focal/ llvm-toolchain-focal-17 main" && \
    apt-get update && \
    apt-get install -y clang-17 llvm-17 llvm-17-dev llvm-17-tools libpolly-17-dev && \
    rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:/usr/lib/llvm-17/bin:$PATH"

ENV LLVM_SYS_170_PREFIX="/usr/lib/llvm-17"

COPY . .

RUN cargo build --verbose

CMD ["bash"]
