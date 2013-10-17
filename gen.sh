#!/bin/bash

if test "x$BINDGEN" == "x"; then
    BINDGEN=bindgen
fi

FFMPEG_INC=ffmpeg/include
export LD_LIBRARY_PATH=$(llvm-config --libdir)
export CPATH=${FFMPEG_INC}

bindgen_opts="-I/usr/lib64/clang/3.3/include -I/usr-builtins -builtins -allow-bitfields"

mkdir -p gen

for lib in avcodec avfilter avformat avdevice swresample swscale; do
    echo bindgen ${lib}...
    $BINDGEN ${bindgen_opts} -match ${lib}.h -l ${lib} -o gen/${lib}.rs ${FFMPEG_INC}/lib${lib}/${lib}.h
done

lib=avutil
echo bindgen ${lib}...
additional_matchs="-match rational.h -match dict.h -match pixfmt.h -match log.h -match samplefmt.h \
                   -match mem.h -match error.h -match mathematics.h"
$BINDGEN ${bindgen_opts} -match ${lib}.h ${additional_matchs} -l ${lib} -o gen/${lib}.rs ${FFMPEG_INC}/lib${lib}/${lib}.h \
                         -include ${FFMPEG_INC}/lib${lib}/samplefmt.h -include ${FFMPEG_INC}/lib${lib}/dict.h

lib=avformat
echo bindgen ${lib}...
additional_matchs="-match avio.h"
$BINDGEN ${bindgen_opts} -match ${lib}.h ${additional_matchs} -l ${lib} -o gen/${lib}.rs ${FFMPEG_INC}/lib${lib}/${lib}.h \
                         -include ${FFMPEG_INC}/lib${lib}/avio.h
