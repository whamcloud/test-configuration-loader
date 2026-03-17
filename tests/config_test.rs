use test_configuration_loader::Config;
use std::collections::HashMap;
use test_configuration_loader::ConfigError;


#[test]
fn test_defaults_only() {
    let env = HashMap::new();

    let cfg = Config::load_from_sources(None, &env);

    assert!(cfg.is_err());
}

#[test]
fn test_file_config() {
    let env = HashMap::new();

    let file = r#"
[server]
host = "0.0.0.0"
port = 9000

[database]
url = "postgres://localhost/db"
pool_size = 20
"#;

    let cfg = Config::load_from_sources(Some(file), &env).unwrap();

    assert_eq!(cfg.server.host, "0.0.0.0");
    assert_eq!(cfg.server.port, 9000);
    assert_eq!(cfg.database.url, "postgres://localhost/db");
    assert_eq!(cfg.database.pool_size, 20);
}

#[test]
fn test_env_override() {
    let mut env = HashMap::new();

    env.insert("APP_SERVER_PORT".into(), "5000".into());
    env.insert("APP_DATABASE_URL".into(), "postgres://env/db".into());

    let file = r#"
[server]
host = "0.0.0.0"
port = 9000

[database]
url = "postgres://file/db"
pool_size = 20
"#;

    let cfg = Config::load_from_sources(Some(file), &env).unwrap();

    assert_eq!(cfg.server.host, "0.0.0.0");
    assert_eq!(cfg.server.port, 5000);
    assert_eq!(cfg.database.url, "postgres://env/db");
    assert_eq!(cfg.database.pool_size, 20);
}

#[test]
fn test_invalid_env() {
    let mut env = HashMap::new();
    env.insert("APP_SERVER_PORT".into(), "abc".into());

    let res = Config::load_from_sources(None, &env);

    assert!(res.is_err());
}

#[test]
fn test_missing_database_url_when_only_defaults_present() {
    let env = HashMap::new();

    let result = Config::load_from_sources(None, &env);

    match result {
        Err(ConfigError::MissingField { field }) => {
            assert_eq!(field, "database.url");
        }
        _ => panic!("expected MissingField for database.url"),
    }
}

#[test]
fn test_env_only_config() {
    let mut env = HashMap::new();
    env.insert("APP_SERVER_HOST".into(), "0.0.0.0".into());
    env.insert("APP_SERVER_PORT".into(), "8081".into());
    env.insert("APP_DATABASE_URL".into(), "postgres://env-only/db".into());
    env.insert("APP_DATABASE_POOL_SIZE".into(), "15".into());

    let cfg = Config::load_from_sources(None, &env).unwrap();

    assert_eq!(cfg.server.host, "0.0.0.0");
    assert_eq!(cfg.server.port, 8081);
    assert_eq!(cfg.database.url, "postgres://env-only/db");
    assert_eq!(cfg.database.pool_size, 15);
}

#[test]
fn test_file_overrides_defaults() {
    let env = HashMap::new();

    let file = r#"
[server]
port = 7000

[database]
url = "postgres://file/db"
"#;

    let cfg = Config::load_from_sources(Some(file), &env).unwrap();

    assert_eq!(cfg.server.host, "127.0.0.1");
    assert_eq!(cfg.server.port, 7000);
    assert_eq!(cfg.database.url, "postgres://file/db");
    assert_eq!(cfg.database.pool_size, 10);
}

#[test]
fn test_env_overrides_file_and_defaults() {
    let mut env = HashMap::new();
    env.insert("APP_SERVER_HOST".into(), "192.168.1.10".into());
    env.insert("APP_SERVER_PORT".into(), "5050".into());
    env.insert("APP_DATABASE_POOL_SIZE".into(), "50".into());

    let file = r#"
[server]
host = "0.0.0.0"
port = 9000

[database]
url = "postgres://file/db"
pool_size = 20
"#;

    let cfg = Config::load_from_sources(Some(file), &env).unwrap();

    assert_eq!(cfg.server.host, "192.168.1.10");
    assert_eq!(cfg.server.port, 5050);
    assert_eq!(cfg.database.url, "postgres://file/db");
    assert_eq!(cfg.database.pool_size, 50);
}

#[test]
fn test_invalid_database_pool_size_in_env() {
    let mut env = HashMap::new();
    env.insert("APP_DATABASE_URL".into(), "postgres://env/db".into());
    env.insert("APP_DATABASE_POOL_SIZE".into(), "xyz".into());

    let result = Config::load_from_sources(None, &env);

    match result {
        Err(ConfigError::InvalidEnv { key, .. }) => {
            assert_eq!(key, "APP_DATABASE_POOL_SIZE");
        }
        _ => panic!("expected InvalidEnv for APP_DATABASE_POOL_SIZE"),
    }
}

#[test]
fn test_invalid_toml_file() {
    let env = HashMap::new();

    let file = r#"
[server
host = "0.0.0.0"
port = 9000
"#;

    let result = Config::load_from_sources(Some(file), &env);

    match result {
        Err(ConfigError::ParseFile { path, .. }) => {
            assert_eq!(path, "config.toml");
        }
        _ => panic!("expected ParseFile error"),
    }
}

#[test]
fn test_validation_fails_when_server_port_is_zero() {
    let env = HashMap::new();

    let file = r#"
[server]
host = "0.0.0.0"
port = 0

[database]
url = "postgres://localhost/db"
pool_size = 20
"#;

    let result = Config::load_from_sources(Some(file), &env);

    match result {
        Err(ConfigError::Validation { message }) => {
            assert!(message.contains("server.port"));
        }
        _ => panic!("expected Validation error for server.port"),
    }
}

#[test]
fn test_validation_fails_when_pool_size_is_zero() {
    let env = HashMap::new();

    let file = r#"
[server]
host = "0.0.0.0"
port = 9000

[database]
url = "postgres://localhost/db"
pool_size = 0
"#;

    let result = Config::load_from_sources(Some(file), &env);

    match result {
        Err(ConfigError::Validation { message }) => {
            assert!(message.contains("database.pool_size"));
        }
        _ => panic!("expected Validation error for database.pool_size"),
    }
}

#[test]
fn test_missing_server_section_still_uses_defaults() {
    let env = HashMap::new();

    let file = r#"
[database]
url = "postgres://localhost/db"
pool_size = 20
"#;

    let cfg = Config::load_from_sources(Some(file), &env).unwrap();

    assert_eq!(cfg.server.host, "127.0.0.1");
    assert_eq!(cfg.server.port, 8080);
    assert_eq!(cfg.database.url, "postgres://localhost/db");
    assert_eq!(cfg.database.pool_size, 20);
}

#[test]
fn test_missing_database_pool_size_uses_default() {
    let env = HashMap::new();

    let file = r#"
[database]
url = "postgres://localhost/db"
"#;

    let cfg = Config::load_from_sources(Some(file), &env).unwrap();

    assert_eq!(cfg.database.url, "postgres://localhost/db");
    assert_eq!(cfg.database.pool_size, 10);
}

#[test]
fn test_env_can_supply_only_missing_database_url() {
    let mut env = HashMap::new();
    env.insert("APP_DATABASE_URL".into(), "postgres://env/db".into());

    let file = r#"
[server]
host = "0.0.0.0"
port = 9000
"#;

    let cfg = Config::load_from_sources(Some(file), &env).unwrap();

    assert_eq!(cfg.server.host, "0.0.0.0");
    assert_eq!(cfg.server.port, 9000);
    assert_eq!(cfg.database.url, "postgres://env/db");
    assert_eq!(cfg.database.pool_size, 10);
}