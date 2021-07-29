use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use serde_json::to_string;
use serde_json::value::Value;

struct Commit1Resp {
    c1out: Vec<u8>,
}

#[tokio::main]
#[test]
async fn http_req() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client.post("http://0.0.0.0:7788/cloud")
        .body("the exact body that is sent")
        .send()
        .await?
        .json::<HashMap<String, String>>().await?;

    println!("resp={:#?}", resp);
    let value_key = resp.get("c1out").unwrap().clone();
    println!("string={:#?}", value_key.to_string());
    let mut obj = Commit1Resp { c1out: vec![] };
    obj.c1out = value_key.into_bytes();
    println!("test={:#?}", obj.c1out);
    println!("test={ }", "success");
    Ok(())
}

#[test]
pub fn open_file(){
    let mut file = std::fs::File::open("/Users/nateyang/Documents/Documents/c2.params").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
    // Ok((contents));
}

// #[tokio::main]
// #[test]
// async fn post() -> Result<HashMap<String, Value>, reqwest::Error> {
//     // post 请求要创建client
//     let client = reqwest::Client::new();
//
//     // 组装header
//     let mut headers = HeaderMap::new();
//     headers.insert("Content-Type", "application/json".parse().unwrap());
//
//     // 组装要提交的数据
//     let mut data = HashMap::new();
//     // data.insert("user", "yyh");
//     // data.insert("password", "https://docs.rs/serde_json/1.0.59/serde_json/");
//
//     // 发起post请求并返回
//     Ok(client.post("http://0.0.0.0:7788/cloud").headers(headers).json(&data).send().await?.json::<HashMap<String, Value>>().await?)
// }
