#!/bin/bash

build32() {

    # 编译 x32
    riscv64-linux-gnu-gcc \
        -march=rv32g \
        -mabi=ilp32 \
        -static \
        -mcmodel=medany \
        -fvisibility=hidden \
        -nostdlib \
        -nostartfiles -T hello.ld \
        -Isifive_e \
        hello.s -o hello
}

run32() {
    # 运行x32
    qemu-system-riscv64 \
        -nographic \
        -machine sifive_e \
        -bios none \
        -kernel hello

}

build32
run32
