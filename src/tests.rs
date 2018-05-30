use super::*;
use std::path::Path;
use std::env;

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
    
    let dirs = read_shim_paths_from_env_vars();
    let nss_shim_path = &dirs[0];
    let boring_runner_path = &dirs[2];

    assert!(Path::new(nss_shim_path).exists(),
            "nss_bogo_shim not found at {}", nss_shim_path);

    let config = TestConfig {
        client_shim: nss_shim_path.clone(),
        server_shim: nss_shim_path.clone(),
        rootdir: boring_runner_path.clone(),
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
    
    let dirs = read_shim_paths_from_env_vars();
    let nss_shim_path = &dirs[0];
    let boring_shim_path = &dirs[1];
    let boring_runner_path = &dirs[2];

    assert!(Path::new(nss_shim_path).exists(),
            "nss_bogo_shim not found at {}", nss_shim_path);
    assert!(Path::new(boring_shim_path).exists(),
            "bssl_shim not found at {}", boring_shim_path);

    let config = TestConfig {
        client_shim: nss_shim_path.clone(),
        server_shim: boring_shim_path.clone(),
        rootdir: boring_runner_path.clone(),
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

    let dirs = read_shim_paths_from_env_vars();
    let nss_shim_path = &dirs[0];
    let boring_shim_path = &dirs[1];
    let boring_runner_path = &dirs[2];

    assert!(Path::new(nss_shim_path).exists(),
            "nss_bogo_shim not found at {}", nss_shim_path);
    assert!(Path::new(boring_shim_path).exists(),
            "bssl_shim not found at {}", boring_shim_path);

    let config = TestConfig {
        client_shim: boring_shim_path.clone(),
        server_shim: nss_shim_path.clone(),
        rootdir: boring_runner_path.clone(),
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


//Reads shim paths from Environment, or returns default (../dist/ and ../boringssl/).
fn read_shim_paths_from_env_vars() -> Vec<String> {
    
    let nss_shim_path = match env::var_os("NSS_SHIM_PATH") {
        Some(val) => val.into_string().unwrap(),
        None => String::from("../dist/Debug/bin/nss_bogo_shim"),
    };
    let boring_root_dir = match env::var_os("BORING_ROOT_DIR") {
        Some(val) => val.into_string().unwrap(),
        None => String::from("../boringssl/"),
    };
    let boring_shim_path = format!("{}build/ssl/test/bssl_shim", &boring_root_dir);
    let boring_runner_path = format!("{}ssl/test/runner/", &boring_root_dir);
    
    vec![nss_shim_path, boring_shim_path, boring_runner_path]
}
