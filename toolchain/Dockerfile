# toolchain/Dockerfile
FROM ubuntu:24.04
RUN apt-get update && apt-get install -y \
    gcc-riscv64-unknown-elf \
    qemu-system-misc \
    python3 python3-pip git make curl ca-certificates
RUN pip3 install --break-system-packages pytest pexpect pytest-timeout

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- -y --default-toolchain stable --profile minimal && \
    rustup target add riscv64gc-unknown-none-elf

