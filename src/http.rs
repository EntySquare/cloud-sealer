use std::collections::HashMap;
use std::fmt::Error;
use std::convert::AsMut;
use std::fs::File;
use std::io::Read;

use resize_slice::ResizeSlice;
use serde_json::to_string;
use serde_json::value::Value;
use unsigned_varint::encode;

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
pub fn open_file() -> Result<String, Error> {
    // let mut file = std::fs::File::open("/Users/nateyang/Documents/Documents/c2.params").unwrap();
    let mut file = std::fs::File::open("/Users/nateyang/Documents/hello.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
    Ok(contents)
}


#[test]
pub fn test_unsigned_varint() {
    // let mut buf2 :  [u8; 32];
    let mut buf = [0; 32];
    let mut buf2 = &mut [0; 32];
    // let mut buf =encode::u64_buffer();
    // encode::u64(n, &mut buf));
    let miner_id: u64 = 23443345;
    let mut prover_id = u642(miner_id, &mut buf);

    for i in 0..32 {
        if i< prover_id.len(){
            buf2[i]=prover_id[i];
        }
    }


    println!("C2 — prover_id: {:?}", buf2);
}

fn clone_into_array<A, T>(slice: &[T]) -> A
    where

        A: Default + AsMut<[T]>,
        T: Clone,

{
    let mut a = A::default();

    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);

    a
}


#[inline]
pub fn u642(number: u64, buf: &mut [u8; 32]) -> &[u8] {
    let mut n = number;
    let mut i = 0;
    for b in buf.iter_mut() {
        *b = n as u8 | 0x80;
        n >>= 7;
        if n == 0 {
            *b &= 0x7f;
            break;
        }
        i += 1
    }
    debug_assert_eq!(n, 0);
    &buf[0..=i]
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
