mod http;

use std::env;
use lazy_static::lazy_static;
use std::env::VarError;
lazy_static! {
  mut eventing : bool = false;
  mut debug : bool = false;
  mut persisting : bool = false;
  mut tmpPath : String = "./tmp";
  mut natsUrl : String = "http://localhost:4222";
  mut sectorDir : String = "pod";
  mut minerIP : String = "127.0.0.1";
  mut minerIDStr : String = "0";
  mut jobNodeName : String = "0";
  mut taskSectorType : String = "";
  mut taskTyp : String = "";
  mut podIp : String = "";
  mut proofType : u64 ;
  mut reserveGiBForSystemAndUnsealedSector : u64 = 500;
  mut copyFullGiB : u64 ;
  mut sectorMinerID : u64 ;
  mut sectorNumber : u64 ;
//  mut params : []byte ;

}
pub const RegisteredSealProof_StackedDrg2KiBV1:u64 = 0;
pub const RegisteredSealProof_StackedDrg32GiBV1:u64 = 3;
pub const RegisteredSealProof_StackedDrg32GiBV1_1:u64 = 8;
pub const SECTOR_TYPE_2k:&str  = "2KiB";
pub const READ_PIECE :&str = "read-piece";
pub const WINNING_POST :&str = "winning-post";
pub const WINDOW_POST :&str = "window-post";
pub fn env_init(){
    let mut key = "EVENTING";
    match env::var(key) {
        Ok(_) => {
            eventing = false;
        },
        Err(_) => eventing = true
    }
    key = "FILTAB_DEBUG";
    match env::var(key) {
        Ok(_) => {
            debug = false;
        },
        Err(_) => debug = true
    }
    key = "PERSISTING";
    match env::var(key) {
        Ok(_) => {
            persisting = false;
        },
        Err(_) => persisting = true
    }
    key = "TMP_PATH";
    match env::var(key) {
        Ok(val) => {
            tmpPath = val;
        },
        Err(_) => tmpPath = "./tmp",
    }
    key = "NATS_SERVER";
    match env::var(key) {
        Ok(val) => {
            natsUrl = val;
        },
        Err(_) => natsUrl = "http://localhost:4222",
    }
    key = "SECTOR_DIR";
    match env::var(key) {
        Ok(val) => {
          sectorDir = val;
        },
        Err(_) => sectorDir = "pod",
    }
    key = "MINER_IP";
    match env::var(key) {
        Ok(val) => {
            minerIP = val;
        },
        Err(_) => {
            if debug {
                minerIP = "127.0.0.1";
            }
        },
    }
    key = "PROOF_TYPE";
    match env::var(key) {
        Ok(val) => {
            if(val == 3){
               proofType= RegisteredSealProof_StackedDrg32GiBV1;
            }
            else if (val == 8) {
               proofType= RegisteredSealProof_StackedDrg32GiBV1_1;
            }
        },
        Err(e) => proofType = RegisteredSealProof_StackedDrg32GiBV1,
    }
    key = "SECTOR_MINER_ID";
    match env::var(key) {
        Ok(val) => {
            minerIDStr = val.clone();
            sectorMinerID = from_str::<u64>(val);
        },
        Err(e) => sectorMinerID = from_str::<u64>("0"),
    }
    key = "SECTOR_NUMBER";
    match env::var(key) {
        Ok(val) => {
            sectorNumber = from_str::<u64>(val);
        },
        Err(e) => sectorNumber = from_str::<u64>("0"),
    }
    key = "TASK_SECTOR_TYPE";
    match env::var(key) {
        Ok(val) => {
            taskSectorType = val;
        },
        Err(e) => taskSectorType = "",
    }
    if &taskSectorType == SECTOR_TYPE_2k {

    };
    key = "TASK_TYPE";
    match env::var(key) {
        Ok(val) => {
            taskTyp = val;
        },
        Err(e) => println!("task not defined : {}", e),
    }
    key = "JOB_POD_NAME";
    match env::var(key) {
        Ok(val) => {
           match env::var(val){
               Ok(addrs) => {
                   podIp = addrs[0];
                   Println!("current pod IP is : ", &podIp)
               },
               Err(e) => panic!(e),
           }
        },
        Err(_) => {
            if &taskTyp == READ_PIECE || &taskTyp == WINDOW_POST || &taskTyp == WINNING_POST {
            } else {
                panic!("fail to seek JOB_POD_NAME : {}");
            }
        },
    }
    key = "RESERVE_GIB_FOR_COPY_SECTOR";
    match env::var(key) {
        Ok(val) => {
            copyFullGiB = from_str::<u64>(val)
        },
        _ => {}
    }
    key = "PARAMS";
    match env::var(key) {
        Ok(val) => {
            println!("params =>{}",val);
            if &taskTyp == WINDOW_POST {
            } else {
                //encode
            }
        },
        _ => {}
    }
    key = "RESERVE_GIB_FOR_SYSTEM_AND_LAST_UNSEALED_SECTOR";
    match env::var(key) {
        Ok(val) => {
            reserveGiBForSystemAndUnsealedSector = from_str::<u64>(val)
        },
        Err(_) => reserveGiBForSystemAndUnsealedSector = 500,
    }
    key = "JOB_NODE_NAME";
    match env::var(key) {
        Ok(val) => {
            jobNodeName = val;
        },
        Err(_) =>{
            if &taskTyp == READ_PIECE {
               println!("JOB_NODE_NAME env have not set")
            }
        }
    }
}
fn main() {
    println!("Hello, world!");
}
