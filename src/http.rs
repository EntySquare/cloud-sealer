// use std::convert::AsMut;
// use std::io::Read;

// use resize_slice::ResizeSlice;
// use serde_json::to_string;
// use serde_json::value::Value;
// use unsigned_varint::encode;

use std::collections::HashMap;
// use reqwest::{Error, Client};
// use bellperson::ConstraintSystem;
// use tokio::io::AsyncSeek;
// use std::fs::File;
// use std::time::Duration;
// use std::thread;
// use tokio_core::reactor::Core;
use hyper::{HeaderMap};
use serde_json::value::Value;

#[tokio::main]
#[test]
async fn http_req() -> Result<(), Box<dyn std::error::Error>> {
    use std::collections::HashMap;
    struct Commit1Resp {
        c1out: Vec<u8>,
    }

    let client = reqwest::Client::new();
    let resp = client
        .post("http://0.0.0.0:7788/cloud")
        .body("the exact body that is sent")
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    println!("resp={:#?}", resp);
    let value_key = resp.get("c1out").unwrap().clone();
    println!("string={:#?}", value_key.to_string());
    let mut obj = Commit1Resp { c1out: vec![] };
    obj.c1out = value_key.into_bytes();
    println!("test={:#?}", obj.c1out);
    println!("test={ }", "success");
    Ok(())
}

#[tokio::main]
#[test]
async fn http_req2() -> Result<(), Box<dyn std::error::Error>> {
    use std::collections::HashMap;

    let client = reqwest::Client::new();
    let resp = client
        .post("http://127.0.0.1:9999/params")
        .body("the exact body that is sent")
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    let value_key = resp.get("Commit1Out").unwrap().clone();
    println!("{}", value_key);
    Ok(())
}

pub async fn post_params(miner_ip: &String, sector_num: &String, task_type: &String) -> Result<HashMap<String, Value>, reqwest::Error> {
    // post 请求要创建client
    let client = reqwest::Client::new();

    // 组装header
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    // 组装要提交的数据
    let mut data = HashMap::new();
    data.insert("SectorNum", sector_num);
    data.insert("TaskType", task_type);

    // 发起post请求并返回
    Ok(client.post(format!("http://{}:9999/params", miner_ip)).headers(headers).json(&data).send().await?.json::<HashMap<String, Value>>().await?)
}

pub async fn post_response(miner_ip: &String, sector_num: &String, task_type: &String, resp: &String) -> Result<HashMap<String, Value>, reqwest::Error> {
    // post 请求要创建client
    let client = reqwest::Client::new();

    // 组装header
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    // 组装要提交的数据
    let mut data = HashMap::new();
    data.insert("SectorNum", sector_num);
    data.insert("TaskType", task_type);
    data.insert("Body", resp);

    // 发起post请求并返回
    Ok(client.post(format!("http://{}:9999/response", miner_ip)).headers(headers).json(&data).send().await?.json::<HashMap<String, Value>>().await?)
}


#[test]
pub fn test_open_file_json() {
    use crate::{Commit2In, api};
    use std::fs::File;
    let res = File::open("./params/c2.params").unwrap();
    let commit2: Commit2In = serde_json::from_reader(res).unwrap();
    let scp1o2: api::enty_proofs_api::SealCommitPhase1Output = serde_json::from_slice(
        base64_url::decode(commit2.phase_1_out.as_str())
            .unwrap()
            .as_slice(),
    )
        .expect("serde_json err 001");

    println!("{:?}", scp1o2)
}
// #[test]
// pub fn open_file() -> Result<String, dyn std::error::Error> {
//     use std::io::Read;
//     // let mut file = std::fs::File::open("/Users/nateyang/Documents/Documents/c2.params").unwrap();
//     let mut file = std::fs::File::open("/Users/nateyang/Documents/hello.txt").unwrap();
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).unwrap();
//     print!("{}", contents);
//     Ok(contents)
// }

// fn clone_into_array<A, T>(slice: &[T]) -> A
//     where
//
//         A: Default + AsMut<[T]>,
//         T: Clone,
//
// {
//     let mut a = A::default();
//
//     <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
//
//     a
// }

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

