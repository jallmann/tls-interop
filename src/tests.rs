use super::*;
use std::path::Path;

// Test the flattener
#[test]
fn flatten_unittest() {
    use flatten::flatten;

    let mut mat = vec![];
    let mut list1 = vec![];
    list1.push(vec![String::from("l1.1.0")]);
    list1.push(vec![String::from("l1.2.0"), String::from("l1.2.1")]);
    mat.push(list1);
    let mut list2 = vec![];
    list2.push(vec![String::from("l2.1.0")]);
    list2.push(vec![String::from("l2.2.1")]);
    list2.push(vec![String::from("l2.2.2")]);
    mat.push(list2);

    let flat = flatten(&mat);
    assert_eq!(6, flat.len());
}

#[test]
fn nss_loopback_simple() {

    assert!(Path::new("../dist/Debug/bin/nss_bogo_shim").exists(),
            "nss_bogo_shim not found at ../dist/Debug/bin/");

    let config = TestConfig {
        client_shim: String::from("../dist/Debug/bin/nss_bogo_shim"),
        server_shim: String::from("../dist/Debug/bin/nss_bogo_shim"),
        rootdir: String::from("../boringssl/ssl/test/runner/"),
        client_writes_first: false,
    };

    let c = TestCase {
        name: String::from("Simple-Connect"),
        server_key: None,
        client_params: None,
        server_params: None,
        client: None,
        server: None,
    };

    let mut results = Results::new();
    run_test_case_meta(&mut results, &config, &c);

    assert_eq!(results.failed, 0);
}

#[test]
fn nss_client_vs_boring_server_simple() {

    assert!(Path::new("../dist/Debug/bin/nss_bogo_shim").exists(),
            "nss_bogo_shim not found at ../dist/Debug/bin/");
    assert!(Path::new("../boringssl/build/ssl/test/bssl_shim").exists(),
            "bssl_shim not found at ../boringssl/build/ssl/test/");

    let config = TestConfig {
        client_shim: String::from("../dist/Debug/bin/nss_bogo_shim"),
        server_shim: String::from("../boringssl/build/ssl/test/bssl_shim"),
        rootdir: String::from("../boringssl/ssl/test/runner/"),
        client_writes_first: true,
    };

    let c = TestCase {
        name: String::from("Simple-Connect"),
        server_key: None,
        client_params: None,
        server_params: None,
        client: None,
        server: None,
    };

    let mut results = Results::new();
    run_test_case_meta(&mut results, &config, &c);

    assert_eq!(results.failed, 0);
}

#[test]
fn nss_server_vs_boring_client_simple() {

    assert!(Path::new("../dist/Debug/bin/nss_bogo_shim").exists(),
            "nss_bogo_shim not found at ../dist/Debug/bin/");
    assert!(Path::new("../boringssl/build/ssl/test/bssl_shim").exists(),
            "bssl_shim not found at ../boringssl/build/ssl/test/");

    let config = TestConfig {
        client_shim: String::from("../boringssl/build/ssl/test/bssl_shim"),
        server_shim: String::from("../dist/Debug/bin/nss_bogo_shim"),
        rootdir: String::from("../boringssl/ssl/test/runner/"),
        client_writes_first: false,
    };

    let c = TestCase {
        name: String::from("Simple-Connect"),
        server_key: None,
        client_params: None,
        server_params: None,
        client: None,
        server: None,
    };

    let mut results = Results::new();
    run_test_case_meta(&mut results, &config, &c);

    assert_eq!(results.failed, 0);
}
