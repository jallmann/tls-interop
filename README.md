
[![Build Status](https://travis-ci.org/jallmann/tls-interop.svg?branch=master)](https://travis-ci.org/jallmann/tls-interop)

Primitive TLS interop Harness
=============================

This program tests interop between two TLS stacks. In order to use it,
each stack needs to be wrapped in a BoringSSL runner-compatible
[shim](https://boringssl.googlesource.com/boringssl/+/master/ssl/test/PORTING.md).
The runner then runs the shims against each other in a variety (currently small)
of configurations).


Basic Execution Instructions
============================
The harness is run as:

```
tls_interop --client [shim-client] --server [shim-server] --rootdir=[path-to-key-files] --test-cases [test-case-descriptions]
```
For instance:

```
tls_interop --client ${NSS_ROOT}/dist/Darwin15.6.0_cc_64_DBG.OBJ/bin/nss_bogo_shim --server ${NSS_ROOT}/dist/Darwin15.6.0_cc_64_DBG.OBJ/bin/nss_bogo_shim --rootdir=${BORINGSSL_ROOT}/ssl/test/runner/ --test-cases cases.json
```

To swap client and server, you need to run it twice.

The run.sh script makes it easier to run a certain configuration of shims and 
test cases, provided the shims for nss, boringssl and openssl executables are 
can be found in the default locations as specified under Cargo Test Instructions.

Example:

```
./run.sh -m loopback -c cipher_cases/all_cipher_cases_71.json -v
```

-v is for verbose output.
-m is for mode, the configuration of shims used as client and server.
-c is for case file, the file with test cases. 

All available modes are: loopback, boring_client, boring_server, ossl_client, ossl_server.


Cargo Test Instructions
============================
Some of the internal rust test cases run with "cargo test" assume readily built
versions of nss, boringssl and openssl being available in the parent directory.
The NSS shim is expected to be found at "../dist/Debug/bin/nss_bogo_shim".  
The BoringSSL shim is expected to be found at "../boringssl/build/ssl/test/bssl_shim".  
The OpenSSL shim is expected to be found at "../openssl/tests/ossl_shim/ossl_shim".

NOTE: OpenSSL needs to be built with the "enable-external-tests" flag. Otherwise
the ossl_shim is not built.

All three default paths can be overwritten by setting the following environment variables:  
NSS_SHIM_PATH = ${NSS_ROOT}/bin/nss_bogo_shim  
BORING_ROOT_DIR = ${BORINGSSL_ROOT}  
OSSL_SHIM_PATH = ${OPENSSL_ROOT}/tests/ossl_shim/ossl_shim  

```
cargo test
```
Runs a set of very basic connection tests between nss and the other two 
shims and additionally all test cases specified in the cases.json file, in each 
available combination of shims.
