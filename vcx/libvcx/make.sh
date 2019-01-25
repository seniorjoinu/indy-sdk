#!/usr/bin/env bash

if [ -z "${LIBINDY_SOURCE_DIR}" ]; then
    echo "Missing LIBINDY_SOURCE_DIR environment variable"
    echo "e.g. ~/projects/indy-sdk/libindy"
    exit 1
fi

make_for_arch() {
    export VCX_SOURCE_DIR=$PWD

    echo "Building libindy for ${ARCH}"
    cd ${LIBINDY_SOURCE_DIR}
    bash android.build.sh -d ${ARCH}

    export LIBINDY_DIR=${PWD}/libindy_${ARCH}/lib
    echo "LIBINDY_DIR=$LIBINDY_DIR"

    echo "Building libvcx for ${ARCH}"
    cd ${VCX_SOURCE_DIR}
    bash android.build.sh -d ${ARCH}
}

export ARCH="arm"
make_for_arch

export ARCH="arm64"
make_for_arch

export ARCH="x86"
make_for_arch
