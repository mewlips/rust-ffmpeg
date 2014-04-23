#!/bin/sh

mkdir -p lib bin

for lib in src/avutil52 src/avcodec54 src/avcodec55 \
           src/avformat54 src/avformat55 src/avfilter3 src/avfilter4 \
           src/avdevice54 src/avdevice55 src/swresample0 src/swscale2; do
    echo Building $lib ...
    rustc -L lib --out-dir lib $lib/lib.rs || exit 1
done

echo Building rust-ffmpeg ...
rustc -L lib --out-dir bin src/rust-ffmpeg/main.rs
