use filecoin_proofs::with_shape;
use serde::{Deserialize, Serialize};
use api::enty_proofs_api::seal_commit_phase2_inner;
// use ffi_toolkit::{catch_panic_response, raw_ptr};
// use std::io::{Read, Write};
use tokio::time::Instant;
use crate::http::{post_params, post_response};
use std::any::Any;
// use crate::api::enty_proofs_api::SealCommitPhase2Output;
// use crate::http::post_response;
// use http::post_params;

mod api;
mod http;
mod structure;
mod types;

#[derive(Debug, Serialize, Deserialize)]
struct Commit2In {
    #[serde(rename = "SectorSize")]
    sector_size: i64,
    #[serde(rename = "SectorNum")]
    sector_num: i64,
    #[serde(rename = "Phase1Out")]
    phase_1_out: String,
}

//POST 1
#[derive(Debug, Serialize, Deserialize)]
pub struct FetchParams<'a> {
    #[serde(rename = "SectorNum")]
    sector_num: &'a str,
    #[serde(rename = "TaskType")]
    task_type: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit2Resp {
    #[serde(rename = "Commit2Out")]
    commit_2_out: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostResp {
    #[serde(rename = "SectorNum")]
    sector_num: String,
    #[serde(rename = "TaskType")]
    task_type: String,
    #[serde(rename = "Body")]
    body: String,
}

//cargo  build --release --no-default-features --features multicore-sdr --features pairing,gpu
#[tokio::main]
async fn main() {
    println!("[cloud-sealer] >>>1:run main");
    let now = Instant::now();
    //
    // let date= (miner_id, sector_number,  miner_ip, task_typ) = structure::my_env::structure_env_test();
    let date = structure::my_env::structure_env();
    let miner_id = date.0;
    let sector_number = date.1;
    let miner_ip = date.2;
    let task_type = date.3;
    println!("[cloud-sealer] >>>2: env date miner_id:{}", miner_id);
    println!("[cloud-sealer] >>>2: env date sector_number:{}", sector_number);
    println!("[cloud-sealer] >>>2: env date miner_ip:{}", miner_ip);
    println!("[cloud-sealer] >>>2: env date task_type:{}", task_type);
    println!("[cloud-sealer] >>>2: success");

    let mut commit_1_out: String = String::new();
    if let Ok(res) = post_params(&miner_ip, &sector_number.to_string(), &task_type).await {
        commit_1_out = res.get("Commit1Out").unwrap().clone().to_string();
    }
    println!("[cloud-sealer] >>>3: post {} Commit1Out.len: {}", format!("http://{}:9999/params", &miner_ip), commit_1_out.len());
    println!("[cloud-sealer] >>>3: success");
    let d: Vec<_> = commit_1_out.split('"').collect(); //去除多余的 "

    let scp1o2: api::enty_proofs_api::SealCommitPhase1Output = serde_json::from_slice(
        base64::decode(d[1]).unwrap().as_slice()
    ).expect("serde_json enty_proofs_api.SealCommitPhase1Output err 001");

    println!("[cloud-sealer] >>>4: json api::SealCommitPhase1Output.registered_proof:{:?}", scp1o2.registered_proof);
    println!("[cloud-sealer] >>>4: json api::SealCommitPhase1Output.registered_proof.vanilla_proofs:{:?}", scp1o2.vanilla_proofs.type_id());
    println!("[cloud-sealer] >>>4: json api::SealCommitPhase1Output.comm_r:{:?}", scp1o2.comm_r.len());
    println!("[cloud-sealer] >>>4: json api::SealCommitPhase1Output.comm_d:{:?}", scp1o2.comm_d.len());
    println!("[cloud-sealer] >>>4: json api::SealCommitPhase1Output.replica_id:{:?}", scp1o2.replica_id);
    println!("[cloud-sealer] >>>4: json api::SealCommitPhase1Output.seed:{:?}", scp1o2.seed.len());
    println!("[cloud-sealer] >>>4: json api::SealCommitPhase1Output.ticket:{:?}", scp1o2.ticket.len());
    println!("[cloud-sealer] >>>4: success");

    //run c2
    let prover_id = types::miner_id_to_prover_id(miner_id);
    println!("[cloud-sealer] >>>5: run fn seal_commit_phase2_inner date miner_id:{:?}", miner_id);
    println!("[cloud-sealer] >>>5: run fn seal_commit_phase2_inner date prover_id:{:?}", prover_id);
    println!("[cloud-sealer] >>>5: run fn seal_commit_phase2_inner date sector_number:{:?}", sector_number);
    let ret = with_shape!(
        u64::from(scp1o2.registered_proof.sector_size()),
        seal_commit_phase2_inner,
        scp1o2,
        prover_id,
        sector_number,
    );

    match ret {
        Ok(output) => {
            let response_rep = base64::encode(output.proof.as_slice()).to_string();
            println!("[cloud-sealer] >>>6: post {} proof.len: {}", format!("http://{}:9999/response", &miner_ip), &response_rep.len());
            if let Ok(res) = post_response(&miner_ip, &sector_number.to_string(), &task_type, &response_rep).await {
                println!("[cloud-sealer] >>>6: post {} return: {:?}", format!("http://{}:9999/response", &miner_ip), res);
            }
        }
        Err(err) => {
            let str = format!("{:?}", err);
            println!("[cloud-sealer] >>>6: err! seal_commit_phase2_inner : {} ", str);
        }
    }
    println!("[cloud-sealer] >>>1: run main end time:{:?}", now.elapsed());
}

// pub fn open_file() -> Result<String, Error> {
//     let mut file = std::fs::File::open("/Users/nateyang/Documents/hello.txt").unwrap();
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).unwrap();
//     Ok(contents)
// }
