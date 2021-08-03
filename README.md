# cloud-sealer

rust sealer built to run filecoin spec remote commit_phase2 at entysnark integrated.

cloud native filecoin sealer built from rust


cloud sealer native seal_commit_phase2

    // invoke such PARAMS
    porep_config: PoRepConfig,
    phase1_output: SealCommitPhase1Output<Tree>,
    prover_id: ProverId,
    sector_id: SectorId,

    pub struct PoRepConfig {
        pub sector_size: SectorSize, //pub struct SectorSize(pub u64);
        pub partitions: PoRepProofPartitions, //pub struct PoRepProofPartitions(pub u8);
        pub porep_id: [u8; 32],
        pub api_version: ApiVersion, //pub enum ApiVersion {V1_0_0, V1_1_0,}
    }
    
    pub struct SealCommitPhase1Output<Tree: MerkleTreeTrait> {
        #[serde(bound(
            serialize = "VanillaSealProof<Tree>: Serialize",
            deserialize = "VanillaSealProof<Tree>: Deserialize<'de>"
        ))]
        pub vanilla_proofs: Vec<Vec<VanillaSealProof<Tree>>>, //pub type VanillaSealProof<Tree> = stacked::Proof<Tree, DefaultPieceHasher>; //pub type DefaultPieceHasher = Sha256Hasher;
        pub comm_r: Commitment, //pub type Commitment = [u8; 32];
        pub comm_d: Commitment, //pub type Commitment = [u8; 32];
        pub replica_id: <Tree::Hasher as Hasher>::Domain, //pub replica_id: <Tree::Hasher as Hasher>::Domain
        pub seed: Ticket, //pub type Ticket = [u8; 32];
        pub ticket: Ticket, //pub type Ticket = [u8; 32];
    

    ProverId = [u8; 32];

    SectorId(u64);


    Result<SealCommitOutput>
