use std::convert::TryInto;
use std::env;
use std::env::VarError;
use std::io::{ Read};

use base64::{decode, encode};
use filecoin_hashers::Hasher;
use filecoin_proofs::{ProverId, SealCommitOutput, SectorSize, constants};
// use filecoin_proofs_api::seal::{SealCommitPhase1Output};
use paired::bls12_381::Fr;

use storage_proofs_core::sector::SectorId;

// use filecoin_proofs_api::seal;

// debug todo
use serde::{Deserialize, Serialize};
use filecoin_proofs::types::VanillaSealProof as RawVanillaSealProof;
use storage_proofs_core::merkle::MerkleTreeTrait;
use filecoin_proofs::constants::{
    SectorShape16KiB, SectorShape16MiB, SectorShape1GiB, SectorShape2KiB, SectorShape32GiB,
    SectorShape32KiB, SectorShape4KiB, SectorShape512MiB, SectorShape64GiB, SectorShape8MiB,
    SECTOR_SIZE_16_KIB, SECTOR_SIZE_16_MIB, SECTOR_SIZE_1_GIB, SECTOR_SIZE_2_KIB,
    SECTOR_SIZE_32_GIB, SECTOR_SIZE_32_KIB, SECTOR_SIZE_4_KIB, SECTOR_SIZE_512_MIB,
    SECTOR_SIZE_64_GIB, SECTOR_SIZE_8_MIB,
};
use filecoin_proofs::{with_shape, Labels as RawLabels};
use anyhow::{bail, ensure, Error, Result};

use crate::http::u642;
use storage_proofs_core::api_version::ApiVersion;

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


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SealCommitPhase1Output {
    pub registered_proof: RegisteredSealProof,
    pub vanilla_proofs: VanillaSealProof,
    pub comm_r: filecoin_proofs::Commitment,
    pub comm_d: filecoin_proofs::Commitment,
    pub replica_id: <filecoin_proofs::constants::DefaultTreeHasher as Hasher>::Domain,
    pub seed: filecoin_proofs::Ticket,
    pub ticket: filecoin_proofs::Ticket,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegisteredSealProof {
    StackedDrg2KiBV1,
    StackedDrg8MiBV1,
    StackedDrg512MiBV1,
    StackedDrg32GiBV1,
    StackedDrg64GiBV1,

    StackedDrg2KiBV1_1,
    StackedDrg8MiBV1_1,
    StackedDrg512MiBV1_1,
    StackedDrg32GiBV1_1,
    StackedDrg64GiBV1_1,
}

impl RegisteredSealProof {
    pub fn version(self) -> ApiVersion {
        use RegisteredSealProof::*;

        match self {
            StackedDrg2KiBV1 | StackedDrg8MiBV1 | StackedDrg512MiBV1 | StackedDrg32GiBV1
            | StackedDrg64GiBV1 => ApiVersion::V1_0_0,
            StackedDrg2KiBV1_1 | StackedDrg8MiBV1_1 | StackedDrg512MiBV1_1
            | StackedDrg32GiBV1_1 | StackedDrg64GiBV1_1 => ApiVersion::V1_1_0,
        }
    }

    pub fn sector_size(self) -> SectorSize {
        use RegisteredSealProof::*;
        let size = match self {
            StackedDrg32GiBV1 | StackedDrg32GiBV1_1 => constants::SECTOR_SIZE_32_GIB,
            StackedDrg64GiBV1 | StackedDrg64GiBV1_1 => constants::SECTOR_SIZE_64_GIB,
            _ => 0,
        };
        SectorSize(size)
    }
    pub fn partitions(self) -> u8 {
        use RegisteredSealProof::*;
        match self {
            StackedDrg2KiBV1 | StackedDrg2KiBV1_1 => *constants::POREP_PARTITIONS
                .read()
                .expect("porep partitions read error")
                .get(&constants::SECTOR_SIZE_2_KIB)
                .expect("invalid sector size"),
            StackedDrg8MiBV1 | StackedDrg8MiBV1_1 => *constants::POREP_PARTITIONS
                .read()
                .expect("porep partitions read error")
                .get(&constants::SECTOR_SIZE_8_MIB)
                .expect("invalid sector size"),
            StackedDrg512MiBV1 | StackedDrg512MiBV1_1 => *constants::POREP_PARTITIONS
                .read()
                .expect("porep partitions read error")
                .get(&constants::SECTOR_SIZE_512_MIB)
                .expect("invalid sector size"),
            StackedDrg32GiBV1 | StackedDrg32GiBV1_1 => *constants::POREP_PARTITIONS
                .read()
                .expect("porep partitions read error")
                .get(&constants::SECTOR_SIZE_32_GIB)
                .expect("invalid sector size"),
            StackedDrg64GiBV1 | StackedDrg64GiBV1_1 => *constants::POREP_PARTITIONS
                .read()
                .expect("porep partitions read error")
                .get(&constants::SECTOR_SIZE_64_GIB)
                .expect("invalid sector size"),
        }
    }


    fn porep_id(self) -> [u8; 32] {
        let mut porep_id = [0; 32];
        let registered_proof_id = self as u64;
        let nonce: u64 = 0;

        porep_id[0..8].copy_from_slice(&registered_proof_id.to_le_bytes());
        porep_id[8..16].copy_from_slice(&nonce.to_le_bytes());
        porep_id
    }
}


impl VanillaSealProof {
    #[allow(clippy::ptr_arg)]
    fn from_raw<Tree: 'static + MerkleTreeTrait>(
        proof: RegisteredSealProof,
        proofs: &Vec<Vec<RawVanillaSealProof<Tree>>>,
    ) -> Result<Self> {
        use std::any::Any;
        use RegisteredSealProof::*;
        match proof {
            StackedDrg2KiBV1 | StackedDrg2KiBV1_1 => {
                if let Some(proofs) =
                Any::downcast_ref::<Vec<Vec<RawVanillaSealProof<SectorShape2KiB>>>>(proofs)
                {
                    Ok(VanillaSealProof::StackedDrg2KiBV1(proofs.clone()))
                } else {
                    bail!("invalid proofs provided")
                }
            }
            StackedDrg8MiBV1 | StackedDrg8MiBV1_1 => {
                if let Some(proofs) =
                Any::downcast_ref::<Vec<Vec<RawVanillaSealProof<SectorShape8MiB>>>>(proofs)
                {
                    Ok(VanillaSealProof::StackedDrg8MiBV1(proofs.clone()))
                } else {
                    bail!("invalid proofs provided")
                }
            }
            StackedDrg512MiBV1 | StackedDrg512MiBV1_1 => {
                if let Some(proofs) =
                Any::downcast_ref::<Vec<Vec<RawVanillaSealProof<SectorShape512MiB>>>>(proofs)
                {
                    Ok(VanillaSealProof::StackedDrg512MiBV1(proofs.clone()))
                } else {
                    bail!("invalid proofs provided")
                }
            }
            StackedDrg32GiBV1 | StackedDrg32GiBV1_1 => {
                if let Some(proofs) =
                Any::downcast_ref::<Vec<Vec<RawVanillaSealProof<SectorShape32GiB>>>>(proofs)
                {
                    Ok(VanillaSealProof::StackedDrg32GiBV1(proofs.clone()))
                } else {
                    bail!("invalid proofs provided")
                }
            }
            StackedDrg64GiBV1 | StackedDrg64GiBV1_1 => {
                if let Some(proofs) =
                Any::downcast_ref::<Vec<Vec<RawVanillaSealProof<SectorShape64GiB>>>>(proofs)
                {
                    Ok(VanillaSealProof::StackedDrg64GiBV1(proofs.clone()))
                } else {
                    bail!("invalid proofs provided")
                }
            }
        }
    }
}

impl<Tree: 'static + MerkleTreeTrait> TryInto<Vec<Vec<RawVanillaSealProof<Tree>>>>
for VanillaSealProof
{
    type Error = Error;

    fn try_into(self) -> Result<Vec<Vec<RawVanillaSealProof<Tree>>>> {
        use std::any::Any;
        use VanillaSealProof::*;

        match self {
            StackedDrg2KiBV1(raw) => {
                if let Some(raw) = Any::downcast_ref::<Vec<Vec<RawVanillaSealProof<Tree>>>>(&raw) {
                    Ok(raw.clone())
                } else {
                    bail!("cannot convert 2kib into different structure")
                }
            }
            StackedDrg8MiBV1(raw) => {
                if let Some(raw) = Any::downcast_ref::<Vec<Vec<RawVanillaSealProof<Tree>>>>(&raw) {
                    Ok(raw.clone())
                } else {
                    bail!("cannot convert 8Mib into different structure")
                }
            }
            StackedDrg512MiBV1(raw) => {
                if let Some(raw) = Any::downcast_ref::<Vec<Vec<RawVanillaSealProof<Tree>>>>(&raw) {
                    Ok(raw.clone())
                } else {
                    bail!("cannot convert 512Mib into different structure")
                }
            }
            StackedDrg32GiBV1(raw) => {
                if let Some(raw) = Any::downcast_ref::<Vec<Vec<RawVanillaSealProof<Tree>>>>(&raw) {
                    Ok(raw.clone())
                } else {
                    bail!("cannot convert 32gib into different structure")
                }
            }
            StackedDrg64GiBV1(raw) => {
                if let Some(raw) = Any::downcast_ref::<Vec<Vec<RawVanillaSealProof<Tree>>>>(&raw) {
                    Ok(raw.clone())
                } else {
                    bail!("cannot convert 64gib into different structure")
                }
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VanillaSealProof {
    StackedDrg2KiBV1(Vec<Vec<RawVanillaSealProof<SectorShape2KiB>>>),
    StackedDrg8MiBV1(Vec<Vec<RawVanillaSealProof<SectorShape8MiB>>>),
    StackedDrg512MiBV1(Vec<Vec<RawVanillaSealProof<SectorShape512MiB>>>),
    StackedDrg32GiBV1(Vec<Vec<RawVanillaSealProof<SectorShape32GiB>>>),
    StackedDrg64GiBV1(Vec<Vec<RawVanillaSealProof<SectorShape64GiB>>>),
}

fn main() {

    // unsafe { env_init(); }
    println!("run main ------------------");
    let res = &std::fs::read("/Users/terrill/sandbox/cloud-sealer/params/c2.params").unwrap();
    let mut scp1o: Result<SealCommitPhase1Output, Error> = serde_json::from_slice(res).map_err(Into::into);
    let mut scp1o2 = scp1o.unwrap().clone();


    // seal_commit_phase2_inner(scp1o.unwrap());
    with_shape!(
        u64::from(scp1o2.registered_proof.sector_size()),
        seal_commit_phase2_inner,
        scp1o2,
    )

    // println!("{:?}", scp1o);
}

fn seal_commit_phase2_inner<Tree: 'static + MerkleTreeTrait>(scp1o: SealCommitPhase1Output) {
    let prover_id = [0; 32];
    let SealCommitPhase1Output {
        vanilla_proofs,
        comm_r,
        comm_d,
        replica_id,
        seed,
        ticket,
        registered_proof,
    } = scp1o;

    let config = filecoin_proofs::PoRepConfig {
        sector_size: registered_proof.sector_size(),
        partitions: filecoin_proofs::PoRepProofPartitions(registered_proof.partitions()),
        porep_id: registered_proof.porep_id(),
        api_version: registered_proof.version(),
    };
    let replica_id: Fr = replica_id.into();

    let co = filecoin_proofs::types::SealCommitPhase1Output {
        vanilla_proofs: vanilla_proofs.try_into().unwrap(),
        comm_r,
        comm_d,
        replica_id: replica_id.into(),
        seed,
        ticket,
    };

    let output = filecoin_proofs::seal_commit_phase2::<Tree>(config, co, prover_id, SectorId::from(0)).unwrap();

    //
    // Ok(SealCommitPhase2Output {
    //     proof: output.proof,
    // });
}

pub fn open_file() -> Result<String, Error> {
    // let mut file = std::fs::File::open("/Users/nateyang/Documents/Documents/c2.params").unwrap();
    let mut file = std::fs::File::open("/Users/nateyang/Documents/hello.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    // print!("{}", contents);
    Ok(contents)
}