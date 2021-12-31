use serde_derive::Deserialize;
use std::path::PathBuf;

fn config_path() -> PathBuf {
    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push("data/config.toml");
    config_path
}

fn toml_dynamic() {
    let config_const_values = {
        let config_text = std::fs::read_to_string(config_path()).unwrap();
        config_text.parse::<toml::Value>().unwrap()
    };

    println!("Original: {:#?}", config_const_values);

    println!(
        "[Postgresql].Database: {}",
        config_const_values
            .get("postgresql")
            .unwrap()
            .get("database")
            .unwrap()
            .as_str()
            .unwrap()
    );
}

#[allow(unused)]
#[derive(Deserialize)]
struct Input {
    xml_file: String,
    json_file: String,
}

#[allow(unused)]
#[derive(Deserialize)]
struct Redis {
    host: String,
}

#[allow(unused)]
#[derive(Deserialize)]
struct Sqlite {
    db_file: String,
}

#[allow(unused)]
#[derive(Deserialize)]
struct Postgresql {
    username: String,
    password: String,
    host: String,
    port: String,
    database: String,
}

#[allow(unused)]
#[derive(Deserialize)]
struct Config {
    input: Input,
    redis: Redis,
    sqlite: Sqlite,
    postgresql: Postgresql,
}

fn toml_static() {
    let config_const_values: Config = {
        let config_text = std::fs::read_to_string(config_path()).unwrap();
        toml::from_str(&config_text).unwrap()
    };

    println!(
        "[Postgresql].Database: {}",
        config_const_values.postgresql.database
    );
}

fn main() {
    println!("------------------------------\n  DYNAMIC");
    toml_dynamic();
    println!("------------------------------\n  STATIC");
    toml_static();
}
