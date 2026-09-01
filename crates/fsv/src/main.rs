use std::{fs, io};

use fsv::lexical_resource::LexicalResource;
use karp_api_client::KarpRedApiClient;

#[tokio::main]
async fn main() {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("Usage: fsv <PATH>");
        std::process::exit(1);
    };
    println!("path={path:?}");
    let file = fs::File::open(&path).unwrap();
    let mut reader = io::BufReader::new(file);

    let resource: LexicalResource = yaserde::de::from_reader(&mut reader).unwrap();
    // dbg!(&resource);

    let client = KarpRedApiClient::new();
    let resp = client.search().query("schlyter").await.unwrap();
    dbg!(&resp);
    for entry in resource.lexicon.entries {}
}
