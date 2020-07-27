extern crate reqwest;
extern crate serde_json;
extern crate tokio;

use serde::Deserialize;
use serde_json::Value;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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

    println!("Preparing the component description file");
    let mut file = File::create("component.json")?;
    file.write_all(body.as_bytes())?;    
    println!("component.json has been created");

    create_component_dir().unwrap();
    create_react_component().unwrap();

    Ok(())
}

fn create_react_component() -> std::io::Result<()>{

    let mut header = String::new();
    header.push_str("import React from 'react' \n");
    header.push_str("import './styles.css' \n");

    let html_element_name = "button";
    let html_element = uppercase_first_letter(html_element_name);

    let component_declaration = format!("export default function {}() {{ \n \t return ( \n", html_element);

    let mut body = String::new();
    body.push_str(&component_declaration);
    body.push_str("\t ); \n }");

    let mut component = String::new();
    component.push_str(&header);
    component.push_str(&body);

    let mut file = File::create("component/button.jsx")?;
    file.write_all(component.as_bytes())?;    

    Ok(())
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect()
    }
}

fn create_component_dir() -> std::io::Result<()> {
    fs::create_dir_all("component")?;
    Ok(())
}
