use std::sync::Mutex;
use std::sync::MutexGuard;
use lazy_static::lazy_static;
use hex::FromHex;
use crate::bcnet::bcmessage::{VERSION, VERSION_END};
use std::collections::HashMap;

impl Transaction

//PRIMERO RECUPERAMOS LA TABLA DE DATOS

/*
//Modelo de un input
pub struct TxInputs {
    pub script_sig: Script, //signature + public key
    pub index: u64, //index of the transaction in between de outputs of the previous transaction
    pub previous_tx: String //Reference to the previous transaction
}

//Modelo de un output
pub strct TxOutputs{
    pub value: u64, //value of transactions-> nº if coins
    pub script_pubKey: Script  //safety for output
    //el script devuelve true o false
}

//Modelo de una transacción
pub struct TransactionDesc {
    pub input: Vec<TxInputs>,
    pub output: Vec<TxOutputs>,
    pub AssociatedBlock: u32
}
*/

//To provide a list of a few transactions explicitilyFreq
pub struct PrefilledTransaction {
    pub idx: usize,
    pub tx: String
}

lazy_static! {
    static ref TEMPLATE_GETTRANSACTION_PAYLOAD: Mutex<Vec<u64>> = Mutex::new(Vec::with_capacity(500));
    //NO SÉ QUE CAPACIDAD
    pub static ref TRANSACTION_ID: Mutex<Vec<u64>> = {
        let mut m = Vec::with_capacity(500)
        Mutex::new(m);
    }
    pub static ref KNOWN_TRANSACTION: Mutex<HashMap<String, TransactionDesc>> = Mutex::new(HashMap::new());
}

let mut transactions = vec![]; //PARA ALMACENAR TODOS LOS DATOS QUE OBTENEMOS SIN NINGÚN FILTRO

//HARÁN FALTA LAS FUNCIONES PARA CLASIFICAR LA INFORMACIÓN Y SABER QUÉ ES CADA COSA
//IRÁN AQUÍ

//Función crear plantilla de una transacción

//obtener la transaccion como tal
pub fn get_gettransaction_data(search_transaction: &str) -> Vec<u8> {
 
    //TEMPLATE_GETBLOCK_PAYLOAD.lock().unwrap().clone()
}

//A partir de id del bloque obtener el de la transaccion que quiero obtener
pub fn short_transaction_ID (block_id: & Vec<u8>) -> &str {


}

//CUAL ES LA DIFERENCIA CON EL DE ARRIBA???
//pub fn get_getheaders_message_payload() -> Vec<u8> {

//mutexGuard!= mutex
pub fn new_transaction(known_transaction: &mut MutexGuard<HashMap<String, transactions>>,transaction_id: &mut Mutex<Vec<u64>>, transaction: String, previous: String ) -> Result<usize, ()> {

    let search_transaction =  known_transaction.get(&transaction);
    let search_previous = known_transaction.get(&previous);

    match search_previous {
        Some(previous_transaction) => {
            match search_transaction {
                None => {
                    let (val, _) = transaction_id.get(previous_transaction.idx).unwrap();
                    transaction_id[previous_transaction.idx] =  (val.to_string(), true);                    // std::mem::replace(&mut blocks_id[previous_block.idx], (val.to_string(), true));
                    transaction_id.insert((previous_transaction.idx+1) as usize, (transaction.clone(), false));

                    let idx = previous_transaction.idx + 1;
                    known_transaction.insert(transaction.clone(), transactions);
                    // eprintln!("Trouvé previous, Pas trouvé block");
                    // eprintln!("{:?}", blocks_id);
                    // eprintln!("{:?}", known_block);

                    Ok(idx)
                }
                _ => {
                    Ok(0)
                }
            }
        }
        _ => {
            match search_transaction {
                Some(found_transactio) => {
                    // eprintln!("Previous {} non trouvé, Block trouvé {}", &previous, &block);
                    let idx = found_transactio.idx;
                    let val = transactions{idx, previous: previous.clone()};
                    known_transaction.insert(transaction.clone(), val);

                    transaction_id.insert(idx, (previous.clone(), true));
                    eprintln!("Previous non {}, Block oui {}", &previous, &transaction);

                    // eprintln!("{:?}", blocks_id);
                    // eprintln!("{:?}", known_block);
                    std::process::exit(1);
                }
                _ => {
                    eprintln!("Previous non {}, Block non {}", &previous, &transaction);
                    // eprintln!("{:?}", blocks_id);
                    // eprintln!("{:?}", known_block);
                    Err(())
                }
            }
        }
    }
}  

pub fn get_block_txn() -> Vec<u8> {
    //serialized BlockTransactionsRequest
    //pub fn ntxid(&self) -> Sha256dHash



    blocktxn//tiene que devolver esto
}

