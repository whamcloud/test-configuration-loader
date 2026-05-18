use std::env;

// Verify that the hard-coded defaults provide sensible server settings.
// Checks `port`, `max_connections` and `timeout_secs` are set as expected.
#[test]
fn defaults_server_values() {
    let d = test_configuration_loader::defaults::defaults();
    assert!(d.server.is_some());
    let s = d.server.unwrap();
    assert_eq!(s.port, Some(8080));
    assert_eq!(s.max_connections, Some(100));
    assert_eq!(s.timeout_secs, Some(30));
}

#[test]
fn merge_overlay_wins_and_none_preserves() {
    let mut base = test_configuration_loader::partial::PartialConfig::default();
    base.server = Some(test_configuration_loader::partial::ServerPartial {
        port: Some(8080),
        max_connections: Some(50),
        timeout_secs: None,
    });

    let mut overlay = test_configuration_loader::partial::PartialConfig::default();
    overlay.server = Some(test_configuration_loader::partial::ServerPartial {
        port: Some(9000),
        max_connections: None,
        timeout_secs: Some(60),
    });

    let merged = test_configuration_loader::merge::merge(base, overlay);
    let s = merged.server.expect("server present");
    assert_eq!(s.port, Some(9000));
    // overlay had None for max_connections, so base value should remain
    assert_eq!(s.max_connections, Some(50));
    assert_eq!(s.timeout_secs, Some(60));
}

// Ensure parsing invalid TOML returns a parse error (ParseFile variant).
#[test]
fn file_toml_parsing_invalid_returns_parse_error() {
    let mut tmp = tempfile::Builder::new().suffix(".toml").tempfile().expect("tempfile");
    use std::io::Write;
    writeln!(tmp, "this is not = valid = toml").unwrap();
    let path = tmp.path().to_str().unwrap().to_string();

    let res = test_configuration_loader::file::from_file(Some(&path));
    match res {
        Err(test_configuration_loader::error::ConfigError::ParseFile { .. }) => {}
        other => panic!("expected ParseFile, got {:?}", other),
    }
}

// Parse a YAML config file successfully into a PartialConfig and verify fields.
#[test]
fn file_yaml_parsing_successful() {
    let mut tmp = tempfile::Builder::new().suffix(".yaml").tempfile().expect("tempfile");
    use std::io::Write;
    let yaml = r#"
database:
  url: "postgres://127.0.0.1/demo"
server:
  port: 7000
"#;
    write!(tmp, "{}", yaml).unwrap();
    let path = tmp.path().to_str().unwrap().to_string();

    let partial = test_configuration_loader::file::from_file(Some(&path)).expect("parse yaml");
    let db = partial.database.expect("database present");
    assert_eq!(db.url.unwrap(), "postgres://127.0.0.1/demo");
    let srv = partial.server.expect("server present");
    assert_eq!(srv.port, Some(7000));
}

// If an env var cannot be parsed (e.g., DC_PORT set to non-numeric),
// env::from_env should return an InvalidEnvVar error containing the var name and value.
#[test]
fn env_invalid_port_returns_invalid_envvar() {
    let key = "DC_PORT";
    let prev = env::var(key).ok();
    env::set_var(key, "not-a-number");

    let res = test_configuration_loader::env::from_env();

    // restore
    if let Some(v) = prev { env::set_var(key, v); } else { env::remove_var(key); }

    match res {
        Err(test_configuration_loader::error::ConfigError::InvalidEnvVar { name, value, .. }) => {
            assert_eq!(name, "DC_PORT");
            assert_eq!(value, "not-a-number");
        }
        other => panic!("expected InvalidEnvVar, got {:?}", other),
    }
}

// Request loading from an explicit path that does not exist; expect FileNotFound.
#[test]
fn config_load_explicit_missing_file_returns_filenotfound() {
    let path = std::env::temp_dir().join(format!("missing_{}_{}.toml", std::process::id(),
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    let path_str = path.to_str().unwrap();

    let res = test_configuration_loader::config::load_with_explicit(Some(path_str));
    match res {
        Err(test_configuration_loader::error::ConfigError::FileNotFound(p)) => {
            assert!(p.to_str().unwrap().contains("missing_"));
        }
        other => panic!("expected FileNotFound, got {:?}", other),
    }
}

// Validate that a server port of 0 is rejected with a Validation error.
#[test]
fn validate_server_port_zero_returns_validation() {
    let mut p = test_configuration_loader::partial::PartialConfig::default();
    p.database = Some(test_configuration_loader::partial::DatabasePartial { url: Some("postgres://x".into()) });
    p.server = Some(test_configuration_loader::partial::ServerPartial { port: Some(0), max_connections: Some(1), timeout_secs: Some(1) });

    let res = test_configuration_loader::validate::from_partial(p);
    match res {
        Err(test_configuration_loader::error::ConfigError::Validation { field, .. }) => {
            assert_eq!(field, "server.port");
        }
        other => panic!("expected Validation, got {:?}", other),
    }
}

// Create a minimal TOML file containing only database.url and ensure
// Config::load picks it up and uses defaults for missing server values.
#[test]
fn config_load_success_with_file() {
    let mut tmp = tempfile::Builder::new().suffix(".toml").tempfile().expect("tempfile");
    use std::io::Write;
    write!(tmp, r#"[database]
url = "postgres://127.0.0.1/prod"
"#).unwrap();
    let path = tmp.path().to_str().unwrap().to_string();

    let cfg = test_configuration_loader::config::load_with_explicit(Some(&path)).expect("load success");
    assert_eq!(cfg.database.url, "postgres://127.0.0.1/prod");
    // server port should come from defaults
    assert_eq!(cfg.server.port, 8080);
}
