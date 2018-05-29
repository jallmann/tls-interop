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
sudo apt-get -qq update
sudo apt-get install gyp -y
sudo apt-get install ninja-build -y
sudo apt-get install zlib1g-dev -y

cd ..
hg clone https://hg.mozilla.org/projects/nspr
hg clone https://hg.mozilla.org/projects/nss
cd nss
./build.sh
pwd
cd ..
ls


# cd ..
# git clone https://github.com/google/boringssl.git
# cd boringssl
# mkdir build
# cd build
# cmake ..
# make
# cd ../..

sudo echo "127.0.0.1 localhost.localdomain" >> /etc/hosts
sudo echo "::1 localhost.localdomain" >> /etc/hosts
export LD_LIBRARY_PATH=./dist/Debug/lib/
cd tls-interop/
pwd
cargo test
