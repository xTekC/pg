#!/bin/sh

TARGET=$1

CROSS_BUILD_TARGETS="\
riscv64gc-unknown-linux-gnu \
aarch64-linux-android \
aarch64-unknown-linux-gnu \
aarch64-unknown-linux-musl \
x86_64-unknown-freebsd \
x86_64-unknown-netbsd"

case " $CROSS_BUILD_TARGETS " in
    *" $TARGET "*)
        cargo install cross
        cross build --locked --target $TARGET --verbose
        ;;
    *)
        cargo build --locked --target $TARGET --verbose
        ;;
esac