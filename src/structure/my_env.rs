// use std::env;

use std::env;

// static SECTOR_TYPE_2k: &'static str = "2KiB";
// static SECTOR_TYPE_32g: &'static str = "32GiB";
#[derive(Debug)]
pub struct MyEnv {
    eventing: bool,
    debug: bool,
    persisting: bool,
    tmp_path: String,
    nats_url: String,
    sector_dir: String,
    miner_ip: String,
    proof_type: i32,
    miner_id_str: i32,
    sector_miner_id: i32,
    sector_number: i32,
    task_sector_type: String,
    task_typ: String,
    pod_ip: String,
    copy_full_gib: u64,
    params: String,
    reserve_gib_for_system_and_unsealed_sector: u64,
    job_node_name: String,
}

pub fn structure_env() -> (u64, u64, String, String,String) {
    let miner_id = match env::var("SECTOR_MINER_ID") {
        Ok(val) => val.parse::<u64>().unwrap(),
        Err(..) => panic!("env SECTOR_MINER_ID is null!!!"),
    };
    let sector_number = match env::var("SECTOR_NUMBER") {
        Ok(val) => val.parse::<u64>().unwrap(),
        Err(..) => 0,
    };
    let miner_ip: String = match env::var("MINER_IP") {
        Ok(val) => val,
        Err(..) => String::from("127.0.0.1"),
    };
    let task_typ = match env::var("TASK_TYPE") {
        Ok(val) => val,
        Err(..) => panic!("env TASK_TYPE is null!!!"),
    };
    let nats_url = match env::var("NATS_SERVER") {
        Ok(val) => val,
        Err(..) => String::from("http://localhost:4222"),
    };
    (miner_id, sector_number, miner_ip, task_typ,nats_url)
}
// pub fn structure_env_test() -> (u64, u64, String, String) {
//     let miner_id = match env::var("SECTOR_MINER_ID") {
//         Ok(val) => val.parse::<u64>().unwrap(),
//         Err(..) => 1000,
//     };
//     let sector_number = match env::var("SECTOR_NUMBER") {
//         Ok(val) => val.parse::<u64>().unwrap(),
//         Err(..) => 0,
//     };
//     let miner_ip = match env::var("MINER_IP") {
//         Ok(val) => val,
//         Err(..) => String::from("127.0.0.1"),
//     };
//     let task_typ = match env::var("TASK_TYPE") {
//         Ok(val) => val,
//         Err(..) => String::from("task_typ_test_test"),
//     };
//     (miner_id, sector_number, miner_ip, task_typ)
// }
/*
env环境
构造数据
*/
// pub(crate) fn structure_env() -> MyEnv {
//     println!("run structure_env ...");
//
//     let eventing = env::var("EVENTING").is_ok();
//     let debug = env::var("FILTAB_DEBUG").is_ok();
//     let persisting = env::var("PERSISTING").is_ok();
//
//     let tmp_path = match env::var("TMP_PATH") {
//         Ok(val) => val,
//         Err(..) => String::from("./tmp"),
//     };
//
//     let nats_url = match env::var("NATS_SERVER") {
//         Ok(val) => val,
//         Err(..) => String::from("http://localhost:4222"),
//     };
//
//     let sector_dir = match env::var("SECTOR_DIR") {
//         Ok(val) => val,
//         Err(..) => String::from("pod"),
//     };
//
//     let miner_ip = match env::var("MINER_IP") {
//         Ok(val) => val,
//         Err(..) => {
//             if debug {
//                 String::from("127.0.0.1")
//             } else {
//                 String::from("")
//             }
//         }
//     };
//
//     let proof_type = match env::var("PROOF_TYPE") {
//         Ok(val) => {
//             let val = val.parse::<i32>().unwrap();
//             match val {
//                 3 => 3,//RegisteredSealProof_StackedDrg32GiBV1
//                 8 => 8,//RegisteredSealProof_StackedDrg32GiBV1_1
//                 4 => 4,//RegisteredSealProof_StackedDrg64GiBV1
//                 9 => 9,//RegisteredSealProof_StackedDrg64GiBV1_1
//                 _ => 0,
//             }
//         }
//         // this is for windowpost / winning
//         Err(..) => 3, //RegisteredSealProof_StackedDrg32GiBV1
//     };
//
//     let (miner_id_str, sector_miner_id) = match env::var("SECTOR_MINER_ID") {
//         Ok(val) => {
//             let val = val.parse::<i32>().unwrap();
//             (val, val)
//         }
//         Err(..) => (0, 0)
//     };
//
//     let sector_number = match env::var("SECTOR_NUMBER") {
//         Ok(val) => val.parse::<i32>().unwrap(),
//         Err(..) => 0,
//     };
//
//     let task_sector_type = match env::var("TASK_SECTOR_TYPE") {
//         Ok(val) => val,
//         Err(..) => String::from(""),//TODO ？？？
//     };
//     // if taskSectorType == SECTOR_TYPE_2k {
//     //     //TODO ？？？
//     // }
//     let task_typ: String;
//     task_typ = env::var("TASK_TYPE").expect("panic! task not defined");
//
//
//     let pod_ip = match env::var("JOB_POD_NAME") {
//         Ok(_) =>
//             String::from(""),
//         Err(..) => {
//             if task_typ == "read-piece" || task_typ == "window-post" || task_typ == "winning-post" {
//                 String::from("")
//             } else {
//                 panic!("env JOB_POD_NAME err 001!");
//             }
//         }
//     };
//     let copy_full_gib = match env::var("RESERVE_GIB_FOR_COPY_SECTOR") {
//         Ok(val) => val.parse::<u64>().unwrap(),
//         Err(..) => 0,
//     };
//
//     let params = match env::var("PARAMS") {
//         Ok(val) => {
//             if task_typ == "window-post" {
//                 String::from("")
//             } else {
//                 val //[]byte - base64
//             }
//         }
//         Err(..) => String::from(""),
//     };
//
//     let reserve_gib_for_system_and_unsealed_sector = match env::var("RESERVE_GIB_FOR_COPY_SECTOR") {
//         Ok(val) => val.parse::<u64>().unwrap(),
//         Err(..) => 500,
//     };
//     //多个阶段都需要获取JOB_NODE_NAME，设定为必须设置的一种环境变量
//     let job_node_name = match env::var("JOB_NODE_NAME") {
//         Ok(val) => val,
//         Err(..) => {
//             if task_typ == "read-piece" {
//                 println!("JOB_NODE_NAME env have not set");
//                 String::from("")
//             } else {
//                 String::from("")
//             }
//         }
//     };
//     MyEnv {
//         eventing,
//         debug,
//         persisting,
//         tmp_path,
//         nats_url,
//         sector_dir,
//         miner_ip,
//         proof_type,
//         miner_id_str,
//         sector_miner_id,
//         sector_number,
//         task_sector_type,
//         task_typ,
//         pod_ip,
//         copy_full_gib,
//         params,
//         reserve_gib_for_system_and_unsealed_sector,
//         job_node_name,
//     }
// }

