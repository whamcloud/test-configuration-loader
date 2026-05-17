use std::env;

use tempfile::NamedTempFile;
use std::io::Write;

#[test]
fn validate_missing_database() {
    let partial = test_configuration_loader::partial::PartialConfig::default();
    let res = test_configuration_loader::validate::from_partial(partial);
    assert!(res.is_err());
}

#[test]
fn load_from_toml_file_and_merge_env() {
    // Create a minimal TOML file with database URL and server port
    let mut tmp = tempfile::Builder::new().suffix(".toml").tempfile().expect("tempfile");
    write!(tmp, r#"[database]
url = "postgres://127.0.0.1/demo"

[server]
port = 8000
"#).unwrap();

    let path = tmp.path().to_str().unwrap().to_string();

    // Ensure env overrides port to 9000
    let key = "DC_PORT";
    let prev = env::var(key).ok();
    env::set_var(key, "9000");

    let cfg = test_configuration_loader::config::load_with_explicit(Some(&path)).expect("load");
    assert_eq!(cfg.server.port, 9000);
    assert_eq!(cfg.database.url, "postgres://127.0.0.1/demo");

    // restore
    if let Some(v) = prev { env::set_var(key, v); } else { env::remove_var(key); }
}
