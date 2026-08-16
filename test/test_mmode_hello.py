# test/test_mmode_hello.py
import pexpect
import pytest

QEMU_CMD = (
    "qemu-system-riscv64 -M virt -smp 1 -m 2G "
    "-nographic -bios build/myhypervisor.elf "
    "-serial mon:stdio"
)


def test_mmode_hello_world():
    child = pexpect.spawn(QEMU_CMD, timeout=30)
    try:
        child.expect("Hello, world from M-mode!")
    finally:
        child.terminate(force=True)
