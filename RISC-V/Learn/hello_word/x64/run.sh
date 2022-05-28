#!/bin/bash

build64() {
    # 编译 x64
    riscv64-linux-gnu-gcc \
        -march=rv64g \
        -mabi=lp64 \
        -static \
        -mcmodel=medany \
        -fvisibility=hidden \
        -nostdlib \
        -nostartfiles -T hello.ld \
        -Isifive_u \
        hello.s -o hello
}

run64() {
    # 运行x64
    qemu-system-riscv64 \
        -nographic \
        -machine sifive_u \
        -bios none \
        -kernel hello
    # 之后需要按 ctrl-a 后按 x 退出 QEMU
}

build64
run64
