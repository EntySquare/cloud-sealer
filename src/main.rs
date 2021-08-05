use std::env;
use std::fs::File;

use filecoin_proofs::with_shape;
use serde::{Deserialize, Serialize};

use api::enty_proofs_api::seal_commit_phase2_inner;

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

fn main() {
    println!("run main ------------------");

    let res = File::open("./params/c2.params").unwrap();
    let commit2: Commit2In = serde_json::from_reader(res).unwrap();
    let scp1o2: api::enty_proofs_api::SealCommitPhase1Output = serde_json::from_slice(
        base64_url::decode(commit2.phase_1_out.as_str())
            .unwrap()
            .as_slice(),
    )
        .expect("serde_json err 001");

    let miner_id = match env::var("SECTOR_MINER_ID") {
        Ok(val) => val.parse::<u64>().unwrap(),
        Err(..) => panic!("env SECTOR_MINER_ID is null!!!"),
    };
    let sector_number = match env::var("SECTOR_NUMBER") {
        Ok(val) => val.parse::<u64>().unwrap(),
        Err(..) => 0,
    };

    let prover_id = types::miner_id_to_prover_id(miner_id);

    with_shape!(
        u64::from(scp1o2.registered_proof.sector_size()),
        seal_commit_phase2_inner,
        scp1o2,
        prover_id,
        sector_number,
    );
}

// pub fn open_file() -> Result<String, Error> {
//     let mut file = std::fs::File::open("/Users/nateyang/Documents/hello.txt").unwrap();
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).unwrap();
//     Ok(contents)
// }
