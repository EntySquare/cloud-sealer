use std::convert::TryInto;

use anyhow::{bail, Error, Result};
// use base64::{decode, encode};
use filecoin_hashers::Hasher;
use filecoin_proofs::constants::{
    SectorShape2KiB, SectorShape32GiB, SectorShape512MiB, SectorShape64GiB, SectorShape8MiB,
};
use filecoin_proofs::types::VanillaSealProof as RawVanillaSealProof;
use filecoin_proofs::{constants, SectorSize};
// use filecoin_proofs_api::seal::{SealCommitPhase1Output};
use paired::bls12_381::Fr;
// DEBUG todo
use serde::{Deserialize, Serialize};
// use crate::http::u642;
use storage_proofs_core::api_version::ApiVersion;
use storage_proofs_core::merkle::MerkleTreeTrait;
use storage_proofs_core::sector::SectorId;

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

impl<Tree: 'static + MerkleTreeTrait> TryInto<Vec<Vec<RawVanillaSealProof<Tree>>>>
    for VanillaSealProof
{
    type Error = Error;

    fn try_into(self) -> Result<Vec<Vec<RawVanillaSealProof<Tree>>>> {
        use std::any::Any;
        use VanillaSealProof::*;
        match self {
            StackedDrg2KiBV1(raw) => {
                if let Some(raw) =
                    <dyn Any>::downcast_ref::<Vec<Vec<RawVanillaSealProof<Tree>>>>(&raw)
                {
                    Ok(raw.clone())
                } else {
                    bail!("cannot convert 2kib into different structure")
                }
            }
            StackedDrg8MiBV1(raw) => {
                if let Some(raw) =
                    <dyn Any>::downcast_ref::<Vec<Vec<RawVanillaSealProof<Tree>>>>(&raw)
                {
                    Ok(raw.clone())
                } else {
                    bail!("cannot convert 8Mib into different structure")
                }
            }
            StackedDrg512MiBV1(raw) => {
                if let Some(raw) =
                    <dyn Any>::downcast_ref::<Vec<Vec<RawVanillaSealProof<Tree>>>>(&raw)
                {
                    Ok(raw.clone())
                } else {
                    bail!("cannot convert 512Mib into different structure")
                }
            }
            StackedDrg32GiBV1(raw) => {
                if let Some(raw) =
                    <dyn Any>::downcast_ref::<Vec<Vec<RawVanillaSealProof<Tree>>>>(&raw)
                {
                    Ok(raw.clone())
                } else {
                    bail!("cannot convert 32gib into different structure")
                }
            }
            StackedDrg64GiBV1(raw) => {
                if let Some(raw) =
                    <dyn Any>::downcast_ref::<Vec<Vec<RawVanillaSealProof<Tree>>>>(&raw)
                {
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

pub fn seal_commit_phase2_inner<Tree: 'static + MerkleTreeTrait>(
    scp1o: SealCommitPhase1Output,
    prover_id: [u8; 32],
    sector_id: u64,
) {
    let sid = SectorId::from(sector_id);
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

    let output = filecoin_proofs::seal_commit_phase2::<Tree>(config, co, prover_id, sid);

    println!("{:?}", output);
    //
    // Ok(SealCommitPhase2Output {
    //     proof: output.proof,
    // });
}
