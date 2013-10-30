#!/bin/sh

FFMPEG_DIR=ffmpeg
FFMPEG_VER=1.2.4

mkdir -p ${FFMPEG_DIR}
cd ${FFMPEG_DIR}
wget -c http://www.ffmpeg.org/releases/ffmpeg-${FFMPEG_VER}.tar.bz2
tar -xvjf ffmpeg-${FFMPEG_VER}.tar.bz2

mkdir -p build
cd build
../ffmpeg-${FFMPEG_VER}/configure --enable-shared --disable-programs --disable-static --prefix=..
make -j8 install
