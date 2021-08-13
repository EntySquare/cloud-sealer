use filecoin_proofs::with_shape;
use serde::{Deserialize, Serialize};
use api::enty_proofs_api::seal_commit_phase2_inner;
use tokio::time::Instant;
use crate::http::{post_params, post_response};
use std::any::Any;
use std::io::Write;
use rustc_serialize::hex::ToHex;

extern crate json;

mod api;
mod http;
mod structure;
mod types;
mod util;

#[derive(Debug, Serialize, Deserialize)]
struct Commit2In {
    #[serde(rename = "SectorSize")]
    sector_size: i64,
    #[serde(rename = "SectorNum")]
    sector_num: i64,
    #[serde(rename = "Phase1Out")]
    phase_1_out: String,
}

//cargo  build --release --no-default-features --features multicore-sdr --features pairing,gpu
#[tokio::main]
async fn main() {
    let now = Instant::now();

    println!("[cloud-sealer] >>>1:run main time:{:?}", &now);

    // let date= (miner_id, sector_number,  miner_ip, task_typ) = structure::my_env::structure_env_test();
    let date = structure::my_env::structure_env();
    let miner_id = date.0;
    let sector_number = date.1;
    let miner_ip = date.2;
    let task_type = date.3;
    let nats_url = date.4;
    println!("[cloud-sealer] >>>2: env date [miner_id:{}] [sector_number:{}] [miner_ip:{}] [task_type:{}]", miner_id, sector_number, miner_ip, task_type);

    let mut env_json = json::JsonValue::new_object();
    env_json["natsUrl"] = nats_url.as_str().into();
    env_json["taskType"] = task_type.as_str().into();
    env_json["minerIDStr"] = miner_id.into();
    env_json["sectorNumber"] = sector_number.into();

    let mut file = std::fs::File::create("./env.json").expect("[cloud-sealer] >>>2: err! create failed!");
    file.write_all(env_json.dump().as_bytes()).expect("[cloud-sealer] >>>2: err! write c2_env.json file !");
    drop(file);

    let mut commit_1_out: String = String::new();
    if let Ok(res) = post_params(&miner_ip, &sector_number.to_string(), &task_type).await {
        commit_1_out = res.get("Commit1Out").unwrap().clone().to_string();
    }
    println!("[cloud-sealer] >>>3: post {} Commit1Out.len: {}", format!("http://{}:9999/params", &miner_ip), commit_1_out.len());

    let d: Vec<_> = commit_1_out.split('"').collect(); //去除多余的 "
    let scp1o2: api::enty_proofs_api::SealCommitPhase1Output = serde_json::from_slice(
        base64::decode(d[1]).unwrap().as_slice()
    ).expect("serde_json enty_proofs_api.SealCommitPhase1Output err 001");

    println!("[cloud-sealer] >>>4: json api::SealCommitPhase1Output [registered_proof:{:?}] [vanilla_proofs:{:?}] [comm_r:{:?}] [comm_d:{:?}] [replica_id:{:?}] [seed:{:?}] [ticket:{:?}]", scp1o2.registered_proof, scp1o2.vanilla_proofs.type_id(), scp1o2.comm_r.len(), scp1o2.comm_d.len(), scp1o2.replica_id, scp1o2.seed.len(), scp1o2.ticket.len());

    //run c2
    let prover_id = types::miner_id_to_prover_id(miner_id);
    println!("[cloud-sealer] >>>5: run fn seal_commit_phase2_inner date [miner_id:{:?}] [sector_number:{:?}] [prover_id:{:?}]", miner_id, sector_number, prover_id);
    let ret = with_shape!(
        u64::from(scp1o2.registered_proof.sector_size()),
        seal_commit_phase2_inner,
        scp1o2,
        prover_id,
        sector_number,
    );

    match ret {
        Ok(output) => {
            println!("[cloud-sealer] >>>5: success");
            let proof16 =  output.proof.to_hex();
            println!("proof_16:{:?}", proof16);

            let mut commit_2_resp = json::JsonValue::new_object();
            commit_2_resp["Commit2Out"] = base64::encode(proof16).into();//16进制
            let commit_2_resp_json = commit_2_resp.dump();
            println!("{}", commit_2_resp_json);
            println!("[cloud-sealer] >>>6: post {} proof.len: {}", format!("http://{}:9999/response", &miner_ip), &output.proof.to_hex().len());

            if let Ok(res) = post_response(&miner_ip, &sector_number.to_string(), &task_type, &base64::encode(commit_2_resp_json.as_bytes())).await {
                // println!("[cloud-sealer] >>>6: post {} return", format!("http://{}:9999/response", &miner_ip), res);
            }

            let mut event = json::JsonValue::new_object();
            event["Body"] = commit_2_resp_json.as_bytes().into();
            event["Head"] = {
                let mut head = json::JsonValue::new_object();
                head["MsgTyp"] = task_type.as_str().into();
                head["SectorNum"] = sector_number.to_string().as_str().into();
                head
            };

            println!("[cloud-sealer] >>>7:  write c2_event.json file...");
            let mut file = std::fs::File::create("./c2_event.json").expect("[cloud-sealer] >>>7: err! create failed!");
            file.write_all(event.dump().as_bytes()).expect("[cloud-sealer] >>>7: err! write c2_event.json file !");

            println!("[cloud-sealer] >>>1: success main end time:{:?}", now.elapsed());
        }
        Err(err) => {
            let str = format!("{:?}", err);
            println!("[cloud-sealer] >>>6: err! seal_commit_phase2_inner : {} ", str);
            let mut event_err = json::JsonValue::new_object();
            // event_err["TaskTyp"] = task_type.as_str().into();
            event_err["Err"] = str.as_str().into();

            let mut file = std::fs::File::create("./c2_event.json").expect("[cloud-sealer] >>>7: err! create failed!");
            file.write_all(event_err.dump().as_bytes()).expect("[cloud-sealer] >>>7: err! write c2_event.json file !");

            println!("[cloud-sealer] >>>1: err! main end time:{:?}", now.elapsed());
        }
    }
}
