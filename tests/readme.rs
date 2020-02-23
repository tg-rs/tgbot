use regex::Regex;
use std::{
    fs::File,
    io::{Error as IoError, Read},
};
use toml::Value;

fn read_file(path: &str) -> Result<String, IoError> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

fn get_crate_version() -> String {
    let manifest = read_file("./Cargo.toml").expect("Failed to get Cargo.toml data");
    let value: Value = manifest.parse().expect("Failed to parse Cargo.toml");
    let version = value["package"]["version"]
        .as_str()
        .expect("Can not get version from Cargo.toml");
    String::from(version)
}

#[test]
fn readme_versions() {
    let version = get_crate_version();
    let readme = read_file("./README.md").expect("Failed to get README.md data");
    for pattern in &[
        r#"https://github\.com/tg-rs/tgbot/tree/([\d\.]+)"#,
        r#"tgbot\s?=\s?"([\d\.]+)""#,
    ] {
        let regex = Regex::new(pattern).expect("Can not create regex");
        for (line_idx, line_data) in readme.lines().enumerate() {
            let line_number = line_idx + 1;
            if let Some(captures) = regex.captures(line_data) {
                let line_version = &captures[1];
                assert_eq!(
                    line_version, version,
                    "Expects version {} at line {} '{}', found {}",
                    version, line_number, line_data, line_version
                );
            }
        }
    }
}
