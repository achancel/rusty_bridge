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
    println!("Your current OS is *not* linux! BAD");
    info!("Your current OS is *not* linux! BAD");
    panic!("Make your sys linux");
}
//endregion

use cli_log::*; // import logging macros
use log::info;
//
use std::env;
use std::thread;

mod net;
use net::listener::*;

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

    println!("Initiating variables done!");
    info!("Initiating variables done!");

    println!("Trying to start listen...");
    info!("Trying to start listen...");

    let params: (String, String) = all.clone();

    let listener = thread::spawn(move || loop {
        listen(socket.try_clone().unwrap(), params.to_owned())
    });

    // let mut input_string = String::new();
    // while input_string != "x" {                             // This is the part that doesn't work right
    //     input_string.clear();                               // First clear the String. Otherwise it will keep adding to it
    //     io::stdin().read_line(&mut input_string).unwrap();  // Get the stdin from the user, and put it in read_string
    //     println!("You wrote {}", input_string);
    //     }

    // let sender = thread::spawn(move || loop {
    //     send(lla.to_owned())
    // });

    listener.join().unwrap();

    print!("\n2");

    info!("Listening done...");

    Ok(())
}