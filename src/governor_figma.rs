use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
pub async fn get_page() -> Result<(), Box<dyn std::error::Error>> {
  let figma_token = "your-figma-token";
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

  println!("Preparing the component description file");
  let mut file = File::create("component.json")?;
  file.write_all(body.as_bytes())?;    
  println!("component.json has been created");

  Ok(())
}
