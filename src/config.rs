use std::fs;
use std::io::prelude::*;
use rustc_serialize::json;
use std::collections::HashMap;

// The config file that corresponds to a test.
#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct TestCaseAgent {
    pub min_version: Option<u32>,
    pub max_version: Option<u32>,
    pub cipher: Option<String>,
    pub flags: Option<Vec<String>>,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
// These are parameters which let us run parametrized tests.
pub struct TestCaseParams {
    pub versions: Option<Vec<i32>>,
    pub ciphers: Option<Vec<String>>
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct TestCase {
    pub name: String,
    pub server_key: Option<String>,
    pub client_params: Option<TestCaseParams>,
    pub server_params: Option<TestCaseParams>,
    pub shared_params: Option<TestCaseParams>,
    pub client: Option<TestCaseAgent>,
    pub server: Option<TestCaseAgent>,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct TestCases {
    pub cases: Vec<TestCase>,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct CipherBlacklist {
    pub nss_blacklist: Option<Vec<String>>,
    pub bssl_blacklist: Option<Vec<String>>,
    pub ossl_blacklist: Option<Vec<String>>,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct CipherBlacklistX {
    pub blacklist: Option<HashMap<String, Vec<String>>>,
}

impl CipherBlacklistX {
    pub fn new() -> CipherBlacklistX {
        CipherBlacklistX {
            blacklist: None,
        }
    }
    
    pub fn init(&mut self) {
        let mut bl = fs::File::open("cipher_blacklist_refactor.json").unwrap();
        let mut bls = String::from("");
        bl.read_to_string(&mut bls)
            .expect("Could not read file to string");
        let blacklist: CipherBlacklistX = json::decode(&bls).expect("Malformed JSON blacklist file.");
        self.blacklist = blacklist.blacklist.clone();
    }
}

impl CipherBlacklist {
    
    pub fn new() -> CipherBlacklist {
        CipherBlacklist {
            nss_blacklist: None,
            bssl_blacklist: None,
            ossl_blacklist: None,
        }
    }
    
    pub fn init(&mut self) {
        let mut bl = fs::File::open("cipher_blacklist.json").unwrap();
        let mut bls = String::from("");
        bl.read_to_string(&mut bls)
            .expect("Could not read file to string");
        let blacklist: CipherBlacklist = json::decode(&bls).expect("Malformed JSON blacklist file.");
        self.nss_blacklist = blacklist.nss_blacklist.clone();
        self.bssl_blacklist = blacklist.bssl_blacklist.clone();
        self.ossl_blacklist = blacklist.ossl_blacklist.clone();
    }
    
    pub fn check(&self, cipher: &str, shim: &str) -> bool {
        if shim.contains("bssl_shim") {
            for c in self.bssl_blacklist.clone().unwrap_or_else(|| {
                vec![]
            }) {
                if c == cipher {
                    return true;
                }
            }
        } else if shim.contains("ossl_shim") {
            for c in self.ossl_blacklist.clone().unwrap_or_else(|| {
                vec![]
            }) {
                if c == cipher {
                    return true;
                }
            }
        } else if shim.contains("nss_bogo_shim"){
            for c in self.nss_blacklist.clone().unwrap_or_else(|| {
                vec![]
            }) {
                if c == cipher {
                    return true;
                }
            }
        }
        return false;
    }
}
