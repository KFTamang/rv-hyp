# toolchain/Dockerfile
FROM ubuntu:24.04
RUN apt-get update && apt-get install -y \
    gcc-riscv64-unknown-elf \
    qemu-system-misc \
    python3 python3-pip git make
RUN pip3 install pytest pexpect

