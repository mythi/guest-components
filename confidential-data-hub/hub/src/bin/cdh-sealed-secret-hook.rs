//use confidential_data_hub::{hub::Hub, storage::volume_type::Storage, CdhConfig, DataHub};
use oci_spec::runtime::Spec;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::{self, Read};
use std::path::Path;

#[derive(Debug, Deserialize)]
struct StdIn {
    annotations: HashMap<String, String>,
    bundle: String,
}

#[tokio::main]
async fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed to read from stdin");

    // Parse JSON
    let bundle: StdIn = serde_json::from_str(&buffer).expect("Invalid JSON");

    let b = Spec::load(Path::new(&bundle.bundle).join("config.json")).expect("read config.json");

    //let r = b.root().clone().unwrap_or_default();
    println!("{:?}", bundle.bundle);
    println!("{:?}", b.root().clone().unwrap_or_default().path());
}
