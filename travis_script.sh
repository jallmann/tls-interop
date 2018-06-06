#!/bin/bash

#apt-get update

#apt-get install git -y
#apt-get install cmake -y
#sudo apt-get install build-essential -y
#sudo apt-get install golang -y
#apt-get install mercurial -y
#sudo apt-get install gyp -y
#sudo apt-get install ninja-build -y
#apt-get install clang -y
#sudo apt-get install g++-6 -y
#sudo apt-get install g++-4.8 -y
#sudo apt-get install zlib1g-dev -y
#apt-get install curl -y

#apt-get autoremove -y
#apt-get clean
#apt-get autoclean
#sudo apt-get install gyp -y

sudo apt-get -qq update
sudo apt-get install ninja-build -y
sudo apt-get install zlib1g-dev -y

cd ..
git clone https://chromium.googlesource.com/external/gyp
cd gyp
./setup.py build
sudo ./setup.py install
cd ..
hg clone https://hg.mozilla.org/projects/nspr
hg clone https://hg.mozilla.org/projects/nss
cd nss
./build.sh
cd ..
git clone https://github.com/google/boringssl.git
cd boringssl
mkdir build
cd build
cmake ..
make
cd ../../tls-interop/

#cargo build
#echo $HOST
#echo $DOMSUF
#cat /etc/hosts
#cargo run -- --client /home/travis/build/jallmann/dist/Debug/bin/nss_bogo_shim --server /home/travis/build/jallmann/boringssl/build/ssl/test/bssl_shim --rootdir /home/travis/build/jallmann/boringssl/ssl/test/runner/ --test-cases cases.json --client-writes-first

cargo test
