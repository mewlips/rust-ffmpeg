#!/bin/sh

TARGET=target
mkdir -p $TARGET

for lib in src/avutil52 src/avcodec54 src/avcodec55 \
           src/avformat54 src/avformat55 src/avfilter3 src/avfilter4 \
           src/avdevice54 src/avdevice55 src/swresample0 src/swscale2; do
    echo Building $lib ...
    rustc -L $TARGET --crate-type dylib --out-dir $TARGET $lib/lib.rs || exit 1
    rustc -L $TARGET --crate-type rlib  --out-dir $TARGET $lib/lib.rs || exit 1
done
