use std::{fs, path::PathBuf};

use schemars::schema_for;

use cgen;

#[test]
fn generate_schemas() {
    let schema = schema_for!(cgen::cfg::Settings);

    let schema_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config.schema.json");

    fs::write(schema_path, serde_json::to_string_pretty(&schema).unwrap()).unwrap();
}
