#!/bin/bash

ROOT_DIR=$TRAVIS_BUILD_DIR/..

cd $ROOT_DIR
git clone https://chromium.googlesource.com/external/gyp
cd gyp
./setup.py build
sudo ./setup.py install
cd $ROOT_DIR
hg clone https://hg.mozilla.org/projects/nspr
hg clone https://hg.mozilla.org/projects/nss
cd nss
hg update -r 606f00fb2cf0
./build.sh
cd $ROOT_DIR
git clone https://github.com/google/boringssl.git
cd boringssl
git checkout -q 2556f8ba60347356f078c753eed2cc65caf5e446
mkdir build
cd build
cmake ..
make -j$(nproc)
cd $ROOT_DIR
git clone -q https://github.com/openssl/openssl.git
cd openssl
git checkout -q 7d38ca3f8bca58bf7b69e78c1f1ab69e5f429dff
./config enable-external-tests
make -j$(nproc)
make install
cd $TRAVIS_BUILD_DIR

cargo test
