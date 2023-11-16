use crate::prelude::*;
mod config;
mod error;
mod prelude;
mod utils;
mod args;

use std::collections::HashMap;
use clap::Parser;
use futures::future::ok;


pub async fn get_ip(url:String)  -> BoxResult<(HashMap<String, String>)> {
    let resp = reqwest::get(url)
        .await?
        .json::<HashMap<String, String>>();
    let res = resp.await?;
    Ok(res)
}


pub async fn baidu() -> BoxResult<()> {
    let resp = reqwest::get("http://api.fanyi.baidu.com/api/trans/vip/translate")
        .await?
        .text()
        .await?;
    println!("{}", resp);
    Ok(())
}


#[tokio::main]
async fn main() -> Result<()> {
    let args =  args::Args::parse();


    let res =  get_ip("https://httpbin.org/ip".to_string()).await;
    if res.is_ok() {
        println!("ok");
        println!("{:?}", res.as_ref().unwrap().get("origin").unwrap());
    } else {
        println!("err");
    }
    Ok(())
}