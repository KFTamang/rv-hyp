# test/test_boot_smoke.py
import pexpect
import pytest

QEMU_CMD = (
    "qemu-system-riscv64 -M virt -smp 4 -m 2G -cpu rv64,h=false "
    "-nographic -bios build/myhypervisor.elf -kernel build/guest_kernel.elf "
    "-serial mon:stdio"
)

@pytest.mark.skip(reason="guest_kernel.elf not implemented yet; tracked as a follow-up (S-mode guest support)")
def test_guest_boots():
    child = pexpect.spawn(QEMU_CMD, timeout=30)
    try:
        child.expect("Hello from S-mode guest")
    finally:
        child.terminate(force=True)

