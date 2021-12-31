use std::path::PathBuf;

fn toml_dynamic() {
    let config_const_values = {
        let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        config_path.push("data/config.toml");
        let config_text = std::fs::read_to_string(&config_path).unwrap();
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

fn main() {
    toml_dynamic();
}
