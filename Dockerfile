# Use the official Ubuntu image as a parent image
FROM ubuntu:20.04

# Avoid any interactive dialog during the build
ENV DEBIAN_FRONTEND=noninteractive

# Set the working directory
WORKDIR /usr/src/safe_llvm

# Install dependencies, add LLVM repository and install LLVM 17 and Polly
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

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add cargo and LLVM to PATH
ENV PATH="/root/.cargo/bin:/usr/lib/llvm-17/bin:$PATH"

# Set LLVM environment variable
ENV LLVM_SYS_170_PREFIX="/usr/lib/llvm-17"

# Copy the current directory contents into the container
COPY . .

# Build the project
RUN cargo build --verbose

# Start a shell when the container starts
CMD ["bash"]
