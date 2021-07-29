use std::convert::TryInto;
use std::env;
use std::env::VarError;
use std::io::{Error, Read};

use anyhow::Result;
use base64::{decode, encode};
use filecoin_proofs::{ProverId, SealCommitOutput};
use filecoin_proofs_api::seal::{SealCommitPhase1Output, SealCommitPhase2Output};
use paired::bls12_381::Fr;
use storage_proofs_core::merkle::MerkleTreeTrait;
use storage_proofs_core::sector::SectorId;

use crate::http::u642;

mod http;

static mut eventing: bool = false;
static mut debug: bool = false;
static mut persisting: bool = false;
static mut tmpPath: String = String::new();
static mut natsUrl: String = String::new();
static mut sectorDir: String = String::new();
static mut minerIP: String = String::new();
static mut minerIDStr: String = String::new();
static mut jobNodeName: String = String::new();
static mut taskSectorType: String = String::new();
static mut taskTyp: String = String::new();
static mut podIp: String = String::new();
static mut proofType: u64 = 0;
static mut reserveGiBForSystemAndUnsealedSector: u64 = 500;
static mut copyFullGiB: u64 = 0;
static mut sectorMinerID: u64 = 0;
static mut sectorNumber: u64 = 0;

static mut params: Vec<u8> = vec![];

pub const RegisteredSealProof_StackedDrg2KiBV1: u64 = 0;
pub const RegisteredSealProof_StackedDrg32GiBV1: u64 = 3;
pub const RegisteredSealProof_StackedDrg32GiBV1_1: u64 = 8;
pub const SECTOR_TYPE_2k: &str = "2KiB";
pub const READ_PIECE: &str = "read-piece";
pub const WINNING_POST: &str = "winning-post";
pub const WINDOW_POST: &str = "window-post";

pub unsafe fn env_init() {
    let mut key = "EVENTING";
    match env::var(key) {
        Ok(_) => unsafe {
            eventing = false;
        },
        Err(_) => eventing = true
    }
    key = "FILTAB_DEBUG";
    match env::var(key) {
        Ok(_) => {
            debug = false;
        }
        Err(_) => debug = true
    }
    key = "PERSISTING";
    match env::var(key) {
        Ok(_) => {
            persisting = false;
        }
        Err(_) => persisting = true
    }
    key = "TMP_PATH";
    match env::var(key) {
        Ok(val) => {
            tmpPath = val.to_owned();
        }
        Err(_) => tmpPath = "./tmp".to_string(),
    }
    key = "NATS_SERVER";
    match env::var(key) {
        Ok(val) => {
            natsUrl = val.to_owned();
        }
        Err(_) => natsUrl = "http://localhost:4222".to_string(),
    }
    key = "SECTOR_DIR";
    match env::var(key) {
        Ok(val) => {
            sectorDir = val.to_owned();
        }
        Err(_) => sectorDir = "pod".to_string(),
    }
    key = "MINER_IP";
    match env::var(key) {
        Ok(val) => {
            minerIP = val.to_owned();
        }
        Err(_) => {
            if debug {
                minerIP = "127.0.0.1".to_string();
            }
        }
    }
    key = "PROOF_TYPE";
    match env::var(key) {
        Ok(val) => {
            if (val == "3") {
                proofType = RegisteredSealProof_StackedDrg32GiBV1;
            } else if (val == "8") {
                proofType = RegisteredSealProof_StackedDrg32GiBV1_1;
            }
        }
        Err(e) => proofType = RegisteredSealProof_StackedDrg32GiBV1,
    }
    key = "SECTOR_MINER_ID";
    match env::var(key) {
        Ok(val) => {
            minerIDStr = val.to_owned();
            sectorMinerID = val.parse::<u64>().unwrap();
        }
        Err(e) => sectorMinerID = "0".parse::<u64>().unwrap(),
    }
    key = "SECTOR_NUMBER";
    match env::var(key) {
        Ok(val) => {
            sectorNumber = val.parse::<u64>().unwrap();
        }
        Err(e) => sectorNumber = "0".parse::<u64>().unwrap(),
    }
    key = "TASK_SECTOR_TYPE";
    match env::var(key) {
        Ok(val) => {
            taskSectorType = val.to_owned();
        }
        Err(e) => taskSectorType = String::from(""),
    }
    if taskSectorType == SECTOR_TYPE_2k {};
    key = "TASK_TYPE";
    match env::var(key) {
        Ok(val) => {
            taskTyp = val.to_owned();
        }
        Err(e) => println!("task not defined : {}", e),
    }
    key = "JOB_POD_NAME";
    match env::var(key) {
        Ok(val) => {
            match env::var(val) {
                Ok(addrs) => {
                    // podIp = addrs;
                    // println!("current pod IP is : ", podIp)
                }
                Err(e) => panic!(e),
            }
        }
        Err(_) => {
            if taskTyp == READ_PIECE || taskTyp == WINDOW_POST || taskTyp == WINNING_POST {} else {
                panic!("fail to seek JOB_POD_NAME : {}");
            }
        }
    }
    key = "RESERVE_GIB_FOR_COPY_SECTOR";
    match env::var(key) {
        Ok(val) => {
            copyFullGiB = val.parse::<u64>().unwrap();
        }
        _ => {}
    }
    key = "PARAMS";
    match env::var(key) {
        Ok(val) => {
            println!("params =>{}", val);
            if taskTyp == WINDOW_POST {} else {
                params = base64::decode(val.clone()).unwrap();
            }
        }
        _ => {}
    }
    key = "RESERVE_GIB_FOR_SYSTEM_AND_LAST_UNSEALED_SECTOR";
    match env::var(key) {
        Ok(val) => {
            reserveGiBForSystemAndUnsealedSector = val.parse::<u64>().unwrap();
        }
        Err(_) => reserveGiBForSystemAndUnsealedSector = 500,
    }
    key = "JOB_NODE_NAME";
    match env::var(key) {
        Ok(val) => {
            jobNodeName = val.to_owned();
        }
        Err(_) => {
            if taskTyp == READ_PIECE {
                println!("JOB_NODE_NAME env have not set")
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    let file_txt = open_file();
    println!("{}", file_txt.unwrap());

    let mut buf = [0; 32];
    let mut buf2 = &mut [0; 32];
    let miner_id: u64 = 1000;
    let mut prover_id = u642(miner_id, &mut buf);

    for i in 0..32 {
        if i < prover_id.len() {
            buf2[i] = prover_id[i];
        }
    }
    let prover_id: ProverId = *buf2;
    println!("{:?}", prover_id);
    let scp1o = serde_json::from_slice(&*file_txt.unwrap().into_bytes()).map_err(Into::into);
    // scp1o.and_then(|o| seal_commit_phase2(o, prover_id.inner, SectorId::from(sector_id)));
    scp1o.and_then(|o| seal_commit_phase2(o, prover_id, SectorId::from(0)));
}

pub fn seal_commit_phase2<Tree: 'static + MerkleTreeTrait>(
    phase1_output: SealCommitPhase1Output,
    prover_id: ProverId,
    sector_id: SectorId,
) -> Result<SealCommitPhase2Output> {
    let SealCommitPhase1Output {
        vanilla_proofs,
        comm_r,
        comm_d,
        replica_id,
        seed,
        ticket,
        registered_proof,
    } = phase1_output;

    let config = registered_proof.as_v1_config();
    let replica_id: Fr = replica_id.into();

    let co = filecoin_proofs::types::SealCommitPhase1Output {
        vanilla_proofs: vanilla_proofs.try_into()?,
        comm_r,
        comm_d,
        replica_id: replica_id.into(),
        seed,
        ticket,
    };

    let output = filecoin_proofs::seal_commit_phase2::<Tree>(config, co, prover_id, sector_id)?;

    Ok(SealCommitPhase2Output {
        proof: output.proof,
    })
}

pub fn open_file() -> Result<String, Error> {
    // let mut file = std::fs::File::open("/Users/nateyang/Documents/Documents/c2.params").unwrap();
    let mut file = std::fs::File::open("/Users/nateyang/Documents/hello.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    // print!("{}", contents);
    Ok(contents)
}