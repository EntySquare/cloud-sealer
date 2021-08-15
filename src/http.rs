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
use std::io::Write;

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
#[tokio::main]
#[test]
async fn test_response() {
    // let strr = String::from("8d4245cc235cab7ba443f37fc8b91a907fb5ddde7d42e5eb6e8d20f1a0de14521b909684e5eb9e9226198b3cd6483c8bb71ebfac17cff9e8d15db55d8cf6738b8172ae8317590c6376ad5e7b00106a1c5788880b0b65ddd137e064501985b1b01007ddd8ef39ab55c3f57173a3c6d56b5b7e556f9301771b01551c381c82ac850a6d2c4074544d3f29141e442534c6e591f6086f917cbcf65cb17358e9e033238a7f370c7bdf040355dbb86b9f9caade06a0e4585743a33a3e1e6206c90f60618318c121bb4167cd78383289d74ef5c0e5ddd0bbd68d3be9228bdfdd6fc295f62aa84598fe85d88b74d0723d02673dcfa07691a27c40898afc6c258c102b63ff3e68a1eea398fd7669f0963aa23cf9ea1d20a3742925ae201c497cb04933587601a1059d3d104212b4bfc818c3d63bf7c7ed8044513ac193a89b672baa6bcddd031f10b164e7366f94c0123f44c3981086af783a30baa86a01ec85101889639f9469bb6c371a65c412ade065a8830a99c3b13efdb2ccf1efea19e6e1c8095a6984457c8f3a38250858b863f421f7807f2d6ba061aaa008b946d406fdf95b250102561c53c22a55752ec2335f19c0c248a02051baf70b614cb2eb14af03ae53b3c670cccc940a350c9dfa143df16788c276462f837ee8381aebf151d92ae7d18213bbe88db710e1190f497888a367dab1917f9330ebf760f47caaeb61792efa7a91883a03b2fee7036fa45583e6e42bb5a557e794077988b50e7aaa876ff20ae1ada292e90cf786135724d29809e480bd21c727fe64b33d460eca5c2c3f5a39cbb005dc397f4490b8aaca11f46013410fa4a4ca6553b0e03292b75a0ddcc0e4953bb03fb9a207996e979da9176752042284e0c2b95997d1b3125adfe219450f6246f02f899fae4727d5a48fea03e07de4343c90c8f76406ef7d46b4cc4a499ba309fa14d981f4cb813decd97f76793de5605727db976d910fcbde7870cddf0977879b2456cab0be18775f6838b8f336db8bd771da1661ffeef172848ba93d589a18b6c3f58c9ad2f5b04e84225c4090f7896b99b0951e8e80775ac6ddcece6ee2ae1037749a770305374e8ec13a566e10f8f3777bedd91fb5e62c3958a56606ba88b02364809869afaf51063c177b74d2a84d98e304c2ea365427fc9a1a44211709add02b16acd4380c2b96b504b45aa9e57b55872cbb3efa861d0c01d8f27eca0274d946383c58e04a23aebf2db8bde47a1aa26fb0bd2aec1de1a2bf5e57d5f87f41cf250906288a07fb7d35a80ce2b197aef9ab82d1bf706e0961ea9cfc4e88a3fcd07081a84e83449b6de182a1fcf0044f46648a94c86293732ed1c0ce40a288ffb671a1a918f2547065f042cd79d042b518992adc9e42c7b187a233c717dc2a5c9849a451bf2e74560941e5e95fa98cb3a41f712728969ca01e031feab52413d03ef32e6efe69e3a24837fac9f4833babe29f32d33a721bad2acae4d09aed17f1e175f6654b7ed2379da8e82fefd70006792b903c73a134b41079863f67f80dafe47a67fa5dda331dfadcd48057388ad732ae41f5a2d473d9ab305bb6bd974562be831666578e738221c410e5d8748d54bc405cc178928dba091e6f198da5b6a684af2af06e68e7456b30002e55693b00a172194855836f59030b6fcb0a55bb53e2534f25411d17468e197b16e80f95e7d83090c111aba021bb30ffb4931c5b55b3c4075fa21dbc5722a255a4be65646278390dbc296c6621890b9c6badd20829cc3d831c8cf78a9d94d0c709776e73d8df7b5409d64d991dadc2de734e4b1ac09990cb509f1e7a6bfe7911e9a329aa70d4518f43dd56cb658b9688742ccfc49b90a0504e0cd1727e5b4dd774debcefa1b9531e281f41811ec04fda5a10deaeea3b9b8aebeac69e7e931f2ecee296e74723e47d755f8715961c581a16636d2d771f66d28b6a025bc87cbe071e7ff6a79d4e2b43a8b8cb167de86b10d39c3cc1776c4fd3510ddb6692385588d4743506a19c065fe3e8d41393c5a5ebd6058f076b368c23f336c1e52d017f76f406683f98bd545304c16196858b844f03c69c3bf911ef8ff79d44346e1d6c9240537990c1aff7db5bfa72a0cbcea73f13ac4e4e3d11d9daea31eaec926a322e5441e9687376579bdbad197177b2bd3445ac8189702deba824baa22de5e5abd0a34762aa152091b5ecbef495f32dd83c63230ca4a9af91638c812aee719ebd7a750cf0a0a227fa8e44b4ebb4bc56b6380ae639f6e5dae37f7383a0448889e0f1919be49dff625ac1551ceb064efebf9b9dfcf80c0cd8770b423bf2167c3003f88f0cf2c900d4532af35332d711bf32816d231ba83619a3d149dd19134319047072f816a4bcf43dfa61ab8cfb82f764e3fc2fd406b41957a89e2c0eb5bd863ff326d0e89ea3d4bbe57ae442d99de80f53dea4818a9400b388b7110d1c28cbaf6e2cef20def68e5f11b58f952be12a8e911edb9c18d332022f0d59cc59bfd61a8b4c1ca081121bbafb205ba3c94967899745682e40356ade76b44e34e1cff095e2f403cc2aa8e531076e70cca064c90f58186c9c012dc42660252354a70708f8ceff372d638ed51805775c8c79c4f0b98c932e28daa2a28873d030f8c26497d9997599e796592997eab3b2febaee7a1ddfa2d1541444536060a876506208a4c281e4753dff492ea8dd101b5f1281542feac20056");
    let miner_ip:String = String::from("127.0.0.1");
    let sector_number:String = String::from("1");
    let task_type:String = String::from("task_type_test");

    let b = "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";
    let b2 = b.as_bytes().to_vec();//out 标准

    let mut commit_2_resp = json::JsonValue::new_object();
    commit_2_resp["Commit2Out"] = base64::encode(String::from_utf8(b2).unwrap()).into();
    let commit_2_resp_json_base64 = base64::encode(commit_2_resp.dump());
    println!("{}",commit_2_resp_json_base64);


    // &base64::encode(String::from_utf8(b2).unwrap())
    // if let Ok(res) = post_response(&miner_ip, &sector_number.to_string(), &task_type, &base64::encode(commit_2_resp_json.as_bytes())).await {

    if let Ok(res) = post_response(&miner_ip, &sector_number, &task_type, &commit_2_resp_json_base64).await {
        // println!("[cloud-sealer] >>>6: post {} return", format!("http://{}:9999/response", &miner_ip), res);
        println!("{:?}", res)
    }
    // println!("{:?}", scp1o2)
}
#[tokio::main]
#[test]
async fn test_event2() {
    let b = "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";
    let b2 = b.as_bytes().to_vec();//out 标准

    let mut commit_2_resp = json::JsonValue::new_object();
    commit_2_resp["Commit2Out"] = base64::encode(String::from_utf8(b2).unwrap()).into();
    let commit_2_resp_json_base64 = base64::encode(commit_2_resp.dump());

    // let miner_ip:String = String::from("127.0.0.1");
    let sector_number:String = String::from("1");
    let task_type:String = String::from("task_type_test");


    let mut event = json::JsonValue::new_object();
    event["Body"] = commit_2_resp_json_base64.into();
    event["Head"] = {
        let mut head = json::JsonValue::new_object();
        head["MsgTyp"] = task_type.as_str().into();
        head["SectorNum"] = sector_number.to_string().as_str().into();
        head
    };

    let mut file = std::fs::File::create("./c2_event.json").expect("[cloud-sealer] >>>7: err! create failed!");
    file.write_all(event.dump().as_bytes()).expect("[cloud-sealer] >>>7: err! write c2_event.json file !");
    // println!("{:?}", scp1o2)
}