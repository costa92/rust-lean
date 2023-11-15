use crate::prelude::*;
mod config;
mod error;
mod prelude;
mod utils;
mod args;

use std::collections::HashMap;



#[tokio::main]
async fn main() ->  Result<()> {
    // let setting = config::config::Setting::new();

    let client = utils::clint::Client::new()?;

    let req = client
        .request(reqwest::Method::GET, "https://httpbin.org/ip")
        .build()?;

    let resp = client.execute(req).await?;

    let body = resp.text().await?;

    println!("body: {:?}", body);

    Ok(())
}
