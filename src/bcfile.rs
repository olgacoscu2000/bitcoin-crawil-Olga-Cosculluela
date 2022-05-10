use std::io::BufReader;
use serde::Deserialize;
use std::fs::{self, File};
use std::io::{LineWriter, stdout, Write};
use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::bcblocks;
use crate::bctransactions;
use chrono::{DateTime, Utc};

lazy_static! { 
    //parametro para poder crear y escribir en el fichero sin tener problemas de memoria 
    pub static ref LOGGER: Mutex<LineWriter<Box<dyn Write + Send>>> = Mutex::new(LineWriter::new(Box::new(stdout())));
    // pub static ref BLOCKS: Mutex<LineWriter<Box<dyn Write + Send>>> = Mutex::new(LineWriter::new(Box::new(File::create("./blocks.raw").unwrap())));
    pub static ref SORTIE:LineWriter<File> = LineWriter::new(File::create("./blocks.raw").unwrap());
}

/// Block storage
#[derive(Debug, Deserialize)]
pub struct Block {
    pub elem: String,
    pub next: bool
}

pub fn load_blocks() {
    println!("file 1");
    eprintln!("Début lecture fichier blocks");
    let file = File::open("./blocks.json").unwrap();
    let blocks: Vec<Block> = ::serde_json::from_reader(BufReader::new(file)).unwrap();

    let mut known_block = bcblocks::KNOWN_BLOCK.lock().unwrap();
    let mut blocks_id = bcblocks::BLOCKS_ID.lock().unwrap();

    let mut idx:usize = 1;
    let mut previous: String = "".to_string();
    for item in blocks {
        // eprintln!("-> {}", item.elem);
        blocks_id.push((item.elem.clone(), item.next));
        known_block.insert(item.elem.clone(), bcblocks::BlockDesc{idx, previous});
        if item.next {
            previous = item.elem;
        } else {
            previous = "".to_string();
        }
        idx+=1;
    }
    eprintln!("Fin lecture fichier blocks");
}

pub fn load_transactions() {
  
    eprintln!("Début lecture fichier transactions");
    let file = File::open("./transactions.json").unwrap();
    let transactions: Vec<Block> = ::serde_json::from_reader(BufReader::new(file)).unwrap();

    let mut known_transaction = bctransactions::KNOWN_TRANSACTION.lock().unwrap();
    let mut transaction_id = bctransactions::TRANSACTION_ID.lock().unwrap();

    let mut idx:usize = 1;
    let mut previous: String = "".to_string();
    for item in transactions {
        // eprintln!("-> {}", item.elem);
        transaction_id.push((item.elem.clone(), item.next));
        known_transaction.insert(item.elem.clone(), bctransactions::transactions);
        if item.next {
            previous = item.elem;
        } else {
            previous = "".to_string();
        }
        idx+=1;
    }
    eprintln!("Fin lecture fichier transactions");
}


pub fn store_blocks(blocks: &Vec<(String, bool)>) -> bool {
    println!("file 2");
    let mut file = LineWriter::new(File::create("./blocks-found.json").unwrap());
    let mut new_blocks = false;
    file.write_all(b"[\n").unwrap();
    for i in 1..blocks.len() {
        let (block, next) = &blocks[i];
        file.write_all(format!("\t {{\"elem\": \"{}\", \"next\": {}}}", block, next).as_ref()).unwrap();
        if i < blocks.len()-1 {
         file.write_all(b",\n").unwrap();
        } else {
         file.write_all(b"\n").unwrap();
        }
        if !new_blocks && !next {
            new_blocks = true;
        }
    }
    file.write_all(b"]").unwrap();
    drop(file);
    fs::rename("./blocks-found.json", "./blocks.json").unwrap();
    new_blocks
}

pub fn store_transactions(transactions: &Vec<(String, bool)>) -> bool {
    
    let mut file = LineWriter::new(File::create("./transactions-found.json").unwrap());
    let mut new_transactions = false;
    file.write_all(b"[\n").unwrap();
    for i in 1..transactions.len() {
        let (transaction, next) = &transactions[i];
        file.write_all(format!("\t {{\"elem\": \"{}\", \"next\": {}}}", transaction, next).as_ref()).unwrap();
        if i < transactions.len()-1 {
         file.write_all(b",\n").unwrap();
        } else {
         file.write_all(b"\n").unwrap();
        }
        if !new_transactions && !next {
            new_transactions = true;
        }
    }
    file.write_all(b"]").unwrap();
    drop(file);
    fs::rename("./transactions-found.json", "./transactions.json").unwrap();
    new_transactions
}


/// Addr storage
pub fn open_logfile(arg_file: Option<&str>) {
    println!("file 3");
    let file: File;
    match arg_file {
        None => panic!("Error parsing file name"),
        Some(f) =>  {
            file = File::create(f).unwrap(); //abre si existe o crea sino la file que le hemos dicho
        }
    }
    //Logger para escribir en el fichero 
    //lock para que solo yo pueda escribir, espero a que nadie mas lo esté haciendo y luego voy yo
    //unwrap para excepciones
    let mut logger = LOGGER.lock().unwrap(); 
    //escribimos en el logger una nueva linea (solo)
    //box::new asigna cierta memoria para luego colocar file
    *logger = LineWriter::new(Box::new(file));             
}

pub fn store_event(msg :&String){
    println!("file 4");
    let mut guard = LOGGER.lock().unwrap();
    guard.write_all(msg.as_ref()).expect("error at logging"); 
    //escribimos en el fichero el evento dado siempre que lo podamos conertir a str (cualquiera vale)
}

pub fn store_version_message(target_address: &String, (_, _, _, _): (u32, Vec<u8>, DateTime<Utc>, String)){
    println!("file 5");
    //TODO: supprimer le &VEc
    let mut msg: String  = String::new();
    msg.push_str(format!("Seed: {} \n", target_address).as_ref());
    // msg.push_str(format!("Seed = {}  ", target_address).as_ref());
    // msg.push_str(format!("version = {}   ", version_number).as_str());
    // msg.push_str(format!("user agent = {}   ", user_agent).as_str());
    // msg.push_str(format!("time = {}  ", peer_time.format("%Y-%m-%d %H:%M:%S")).as_str());
    // msg.push_str(format!("now = {}  ", Into::<DateTime<Utc>>::into(SystemTime::now()).format("%Y-%m-%d %H:%M:%S")).as_str());
    // msg.push_str(format!("since = {:?}  ",SystemTime::now().duration_since(SystemTime::from(peer_time)).unwrap_or_default() ).as_str());
    // msg.push_str(format!("services = {:?}\n", services ).as_str());
    store_event(&msg);
}
