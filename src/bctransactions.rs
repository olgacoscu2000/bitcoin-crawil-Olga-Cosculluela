use std::sync::Mutex;
use std::sync::MutexGuard;
use lazy_static::lazy_static;
use hex::FromHex;
use crate::bcnet::bcmessage::{VERSION, VERSION_END};
use std::collections::HashMap;


// pub struct TransactionDesc {
//     pub idx: usize,
//     pub previous: String
// }

lazy_static! {
    // static ref TEMPLATE_MESSAGE_PAYLOAD: Mutex<Vec<u8>> = Mutex::new(Vec::with_capacity(105));
    static ref TEMPLATE_GETBLOCK_TXN: Mutex<Vec<u8>> = Mutex::new(Vec::with_capacity(197));
    // pub static ref BLOCKS_ID: Mutex<Vec<(String, bool)>> = {
    //     let mut m = Vec::with_capacity(5);
    //     m.push((String::from("0000000000000000000000000000000000000000000000000000000000000000"), false));
    //     Mutex::new(m)
    // };
    // pub static ref KNOWN_BLOCK: Mutex<HashMap<String, BlockDesc>> = Mutex::new(HashMap::new());
}

pub fn get_getblock_txn() -> Vec<u8> {
    TEMPLATE_GETBLOCK_TXN.lock().unwrap().clone()
}