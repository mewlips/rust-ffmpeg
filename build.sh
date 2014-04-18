#!/bin/sh

mkdir -p lib bin

for lib in src/{avutil52,avcodec{54,55},avformat{54,55},avfilter{3,4},avdevice{54,55},swresample0,swscale2}; do
    echo Building $lib ...
    rustc -L lib --out-dir lib $lib/lib.rs || exit 1
done

echo Building rust-ffmpeg ...
rustc -L lib --out-dir bin src/rust-ffmpeg/main.rs
