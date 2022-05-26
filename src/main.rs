///*
//region checking OS
//This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn current_os() {
    println!("Your current OS is linux. OK");
    info!("Your current OS is linux. OK");
}
// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn current_os() {
    println!("Your current OS is *not* linux! Bad but can work");
    info!("Your current OS is *not* linux! Bad but can work");
}
//endregion 
//*/

use cli_log::*; // import logging macros
use log::info;

use std::env;
use std::thread;

mod net;
use net::net_io::*;

mod parser;
use parser::parser::*;

/// The main function of the program.
/// 
/// Returns:
/// 
/// A tuple of UdpSocket and &str
fn main() -> Result<(), Box<dyn std::error::Error>> {

    current_os();
    init_cli_log!("rusty");

    println!("Initiating variables...");
    info!("Initiating variables...");

    let arguments_in_string: Vec<String> = env::args().collect();

    let all: (String, String) = get_parametrs(arguments_in_string); //GET PORT AND MODE
    let socket: std::net::UdpSocket = create_socket(all.0.parse::<u32>().unwrap()).unwrap();
    let sock = socket.try_clone().unwrap();
    let receiver: String = "127.0.0.1:5055".to_string();

    println!("Initiating variables done!");
    info!("Initiating variables done!");

    println!("Trying to start listen...");
    info!("Trying to start listen...");

    let params: (String, String) = all.clone();
    
    thread::spawn(move || loop {
        listen(sock.try_clone().unwrap(), params.to_owned())
    });

    // Get the stdin from the user, and put it in read_string
    let sender = thread::spawn(move || loop { net::net_io::forward(socket.try_clone().unwrap(), receiver.clone()) });

    sender.join().unwrap();

    info!("Listening done...");

    Ok(())
}



    // let mut input_string = String::new();

