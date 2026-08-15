# Makefile
build:
	docker run --rm -v $(PWD):/work -w /work toolchain-image \
	    make -C src/ TARGET=riscv64

test: build
	docker run --rm -v $(PWD):/work -w /work toolchain-image \
	    pytest test/ -v --timeout=60

