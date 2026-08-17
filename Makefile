# Makefile
build:
	docker run --rm -v $(PWD):/work -w /work toolchain-image \
	    sh -c "make -C src/ TARGET=riscv64 && make -C guest/ TARGET=riscv64"

test: build
	docker run --rm -v $(PWD):/work -w /work toolchain-image \
	    pytest test/ -v --timeout=60

