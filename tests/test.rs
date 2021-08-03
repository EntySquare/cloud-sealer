use std::convert::TryInto;
use std::io::{Read, Seek, Write};
use std::path::{Path, PathBuf};

use anyhow::{bail, ensure, Error, Result};
use bellperson::bls::Fr;
use filecoin_hashers::Hasher;

use filecoin_proofs_v1::constants::{
    SectorShape16KiB, SectorShape16MiB, SectorShape1GiB, SectorShape2KiB, SectorShape32GiB,
    SectorShape32KiB, SectorShape4KiB, SectorShape512MiB, SectorShape64GiB, SectorShape8MiB,
    SECTOR_SIZE_16_KIB, SECTOR_SIZE_16_MIB, SECTOR_SIZE_1_GIB, SECTOR_SIZE_2_KIB,
    SECTOR_SIZE_32_GIB, SECTOR_SIZE_32_KIB, SECTOR_SIZE_4_KIB, SECTOR_SIZE_512_MIB,
    SECTOR_SIZE_64_GIB, SECTOR_SIZE_8_MIB,
};
use filecoin_proofs_v1::types::MerkleTreeTrait;
use filecoin_proofs_v1::types::VanillaSealProof as RawVanillaSealProof;
use filecoin_proofs_v1::{with_shape, Labels as RawLabels};
use serde::{Deserialize, Serialize};

use crate::{
    AggregateSnarkProof, Commitment, PieceInfo, ProverId, RegisteredAggregationProof,
    SectorId, Ticket, UnpaddedByteIndex, UnpaddedBytesAmount,
};

#[test]
pub fn test_unsigned_varint() {
    let pid = gen_prover_id(1000);
    println!("C2 — prover_id: {:?}", pid);
}

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct SealCommitPhase1Output {
//     pub registered_proof: RegisteredSealProof,
//     pub vanilla_proofs: VanillaSealProof,
//     pub comm_r: Commitment,
//     pub comm_d: Commitment,
//     pub replica_id: <filecoin_proofs_v1::constants::DefaultTreeHasher as Hasher>::Domain,
//     pub seed: Ticket,
//     pub ticket: Ticket,
// }

/// Available seal proofs.
/// Enum is append-only: once published, a `RegisteredSealProof` value must never change.
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

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub enum VanillaSealProof {
//     StackedDrg2KiBV1(Vec<Vec<RawVanillaSealProof<SectorShape2KiB>>>),
//     StackedDrg8MiBV1(Vec<Vec<RawVanillaSealProof<SectorShape8MiB>>>),
//     StackedDrg512MiBV1(Vec<Vec<RawVanillaSealProof<SectorShape512MiB>>>),
//     StackedDrg32GiBV1(Vec<Vec<RawVanillaSealProof<SectorShape32GiB>>>),
//     StackedDrg64GiBV1(Vec<Vec<RawVanillaSealProof<SectorShape64GiB>>>),
// }

#[test]
pub fn calculate_c2() {
    let scp1o: http::SealCommitPhase1Output  = serde_json::from_slice(&std::fs::read("/Users/guodayang/code-work/file/c2.params").unwrap()).unwrap();
    println!("{}",scp1o);
    // let SealCommitPhase1Output {
    //     vanilla_proofs,
    //     comm_r,
    //     comm_d,
    //     replica_id,
    //     seed,
    //     ticket,
    //     registered_proof,
    // } = scp1o;

    // let config = registered_proof.as_v1_config();
    // let replica_id: Fr = replica_id.into();
    //
    // let co = filecoin_proofs_v1::types::SealCommitPhase1Output {
    //     vanilla_proofs: vanilla_proofs.try_into()?,
    //     comm_r,
    //     comm_d,
    //     replica_id: replica_id.into(),
    //     seed,
    //     ticket,
    // };
    // println!("{:?}", scp1o);
    // unsafe { seal_commit_phase2(scp1o, prover_id, SectorId::from(sectorNumber.clone())); }
}

#[inline]
pub fn gen_prover_id(miner_id: u64) -> [u8; 32] {
    let mut buf = [0; 32];
    let mut prover_id = uvarint(miner_id, &mut buf);
    let mut buf2 = &mut [0; 32];
    for i in 0..32 {
        if i < prover_id.len() {
            buf2[i] = prover_id[i];
        }
    };
    *buf2
}

#[inline]
pub fn uvarint(number: u64, buf: &mut [u8; 32]) -> &[u8] {
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