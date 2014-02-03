#!/bin/sh

mkdir -p lib bin

for lib in src/{avutil,avcodec,avformat,avfilter,avdevice,swresample,swscale}; do
    echo Building $lib ...
    rustc -L lib --out-dir lib $lib/lib.rs
done

echo Building rust-ffmpeg ...
rustc -L lib --out-dir bin src/rust-ffmpeg/main.rs
