#![allow(missing_docs)]
use std::fs::read_to_string;

use regex::Regex;
use toml::Value;

fn get_crate_version() -> String {
    let manifest = read_to_string("./Cargo.toml").expect("Failed to get Cargo.toml data");
    let value: Value = toml::from_str(&manifest).expect("Failed to parse Cargo.toml");
    let version = value["package"]["version"]
        .as_str()
        .expect("Can not get version from Cargo.toml");
    String::from(version)
}

#[test]
fn readme_versions() {
    let version = get_crate_version();
    let readme = read_to_string("./README.md").expect("Failed to get README.md data");
    for pattern in &[
        r"https://github\.com/tg-rs/tgbot/tree/([\d\.]+)",
        r#"tgbot\s?=\s?"([\d\.]+)""#,
    ] {
        let regex = Regex::new(pattern).expect("Can not create regex");
        for (line_idx, line_data) in readme.lines().enumerate() {
            let line_number = line_idx + 1;
            if let Some(captures) = regex.captures(line_data) {
                let line_version = &captures[1];
                assert_eq!(
                    line_version, version,
                    "Expects version {version} at line {line_number} '{line_data}', found {line_version}"
                );
            }
        }
    }
}
