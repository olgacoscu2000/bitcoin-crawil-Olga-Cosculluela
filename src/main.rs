// mod bcmessage;
mod bcblocks;
mod bcfile;
mod bcnet;
mod bcpeers;
mod bctransactions;

use clap::{Arg, App};
use std::sync::mpsc;
use std::sync::atomic::Ordering;

use std::thread;
use std::process;

use std::time::{Duration, SystemTime};

const CHECK_TERMINATION_TIMEOUT:Duration = Duration::from_secs(5);
const THREADS: u64 = 500;
const MESSAGE_CHANNEL_SIZE: usize = 100000;


fn main() {
    println!("main");
    bcfile::load_blocks();
    println!("fin load blocks");
    bcblocks::create_block_message_payload(&bcblocks::BLOCKS_ID.lock().unwrap());
    println!("fin create blcok msg payload");

    // eprintln!("{}", hex::encode(bcblocks::get_getblock_message_payload()));
    // eprintln!("{}", hex::encode(bcblocks::get_getheaders_message_payload()));
    // std::process::exit(1);

    // eprintln!("{:?}", known_block);
    // eprintln!("{:?}", bcblocks::BLOCKS_ID.lock().unwrap());
    // std::process::exit(1);

    let (address_channel_sender, address_channel_receiver) = mpsc::channel(); //enviar mensajes entre sender y receiver de manera asincrona
    let (connecting_start_channel_sender, connecting_start_channel_receiver) = chan::sync(MESSAGE_CHANNEL_SIZE);//canal síncrono con un buffer de ese tamaño

    let start_adress = parse_args();

    println!("fin paarse args");
    address_channel_sender.send(start_adress).unwrap();//enviamos la direccion por el canal que hemos creado
    //hace como un fork para que ejecute esa función todo el rato, que es la que para el programa
    thread::spawn(move || { check_pool_size(SystemTime::now()); }); //no se ha podido enviar en ese tiempo

    // println!("fin check pool size");
   
    // //para crear los 500 procesos diferentes que trabajan a la vez para tratar con las addr
    for i in 0..THREADS { // de 0 a 500 clonamos las direcciones, sino damos error
        let sender = address_channel_sender.clone();
        println!("sender clone");
        let recv = connecting_start_channel_receiver.clone();
        println!("connecting start clone");
        thread::spawn(move || { bcnet::handle_one_peer(recv, sender, i);});
        println!("thread");
    }

    loop { //bucle infinito
        let new_peer: String = address_channel_receiver.recv().unwrap(); //el nuevo cliente es la direccion recibida
        println!("new peer recv");
        connecting_start_channel_sender.send(new_peer); // mandamos por el canal al sender la direccion del nuevo cliente
        println!("fin send connecting");
        //sumamos 1 al número de addr que tenemos que probar-> es lo que hace fetch_add(1, Ordering::Relaxed)-> no rayarse más
        bcpeers::NB_ADDR_TO_TEST.fetch_add(1, Ordering::Relaxed);//genera ids unicos para cada thread
        //read modify write opertaino
    }
}  

//unwrap es para errores-> panic
fn check_pool_size(start_time: SystemTime ){
    println!("main 1");
    loop {
        thread::sleep(CHECK_TERMINATION_TIMEOUT);

        bcpeers::get_peers_status(); //obtiene el estado
        if bcpeers::NB_ADDR_TO_TEST.load(Ordering::Relaxed) < 1 { //ha habido alguna excepcion, salimos
            let time_spent = SystemTime::now().duration_since(start_time).unwrap_or_default();
            println!("POOL Crawling ends in {:?} ", time_spent);
            process::exit(0);
        }
    }
}

fn parse_args() -> String {
    println!("main 2");
    let matches = App::new("BC crawl") //represetacion de la linea de comandos, aqui tenemos los comandos posibles
        .version("1.0.0")
        .author("Jazmin Ferreiro  <jazminsofiaf@gmail.com>, Stephane Frenot <stephane.frenot@insa-lyon.fr>")
        .arg(Arg::with_name("file") //añade opcion en el comando
            .short("-o")
            .long("output")
            .takes_value(true)
            .required(true)
            .help("output file name for crawl"))
        .arg(Arg::with_name("address")
            .short("-s")
            .long("address")
            .takes_value(true)
            .required(true)
            .help(" Initial address for crawling. Format [a.b.c.d]:ppp"))
        .get_matches();

    let arg_address = matches.value_of("address").unwrap_or_else(|| {
        panic!("Error parsing address argument");
        }
    );

    bcfile::open_logfile(matches.value_of("file"));
    String::from(arg_address)
}
