extern crate reqwest;
extern crate serde_json;
extern crate tokio;

use serde::Deserialize;
use serde_json::Value;

use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let figma_token = "47459-dc550ef0-84e2-4bdc-8c02-65d223b2f6fb";
    let _figma_node_id = "your-targeted-node-id";

    let res = reqwest::Client::new()
        .get("https://api.figma.com/v1/files/iRGl920bR0gkzDcLazgf0D/")
        .header(
            "X-FIGMA-TOKEN",
            figma_token
        )
        .send()
        .await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    let mut file = File::create("component.json")?;
    file.write_all(body.as_bytes())?;    

    Ok(())
}
