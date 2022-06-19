use std::error::Error;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Origin {
    origin: String
}


async fn fetch() -> Result<(), Box<dyn std::error::Error>> {

    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<Origin>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

#[tokio::main]
async fn main(){

   fetch().await;
}
