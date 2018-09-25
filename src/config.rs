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
