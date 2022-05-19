use chrono::{Timelike, Utc};
use cli_log::*; // import logging macros
use log::{error, info};
//
use std::env;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::time::SystemTime;

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

/// It creates a UDP socket, binds it to the address and port specified in the config file, and then
/// starts listening for incoming messages
/// 
/// Arguments:
/// 
/// * `all`: (UdpSocket, &str)
fn start_listening(all: (UdpSocket, &str)) {
    let mut buf = vec![0; 10];
    let mut result: Vec<u8> = Vec::new();
    let _now = SystemTime::now();
    let socket: UdpSocket = all.0;
    let mode: &str = all.1;

    println!("start_looping!");
    info!("start_looping!");

    match mode {
        "f" | "file" => {
            println!("Listening...");
            info!("Listening...");

            loop {
                let _now = Utc::now();

                match socket.recv_from(&mut buf) {
                    Ok((number_of_bytes, src_addr)) => {
                        println!("received bytes: {:?} from {:?}", buf, src_addr);
                        result = Vec::from(&buf[0..number_of_bytes]);
                        while result.last() == Some(&10) || result.last() == Some(&0) {
                            result.pop();
                        }
                    }
                    Err(fail) => println!("failed listening {:?}", fail),
                }
                let display_result = result.clone();
                let result_str = String::from_utf8(display_result).unwrap();

                let mut file = match std::fs::OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open("messages.txt")
                {
                    Ok(file) => file,
                    Err(error) => match error.kind() {
                        ErrorKind::NotFound => match File::create("messages.txt") {
                            Ok(fc) => fc,
                            Err(e) => panic!("Problem creating the file: {:?}", e),
                        },
                        _ => {
                            panic!("!");
                        }
                    },
                };
                write!(
                    file,
                    "Received message - time : {:02}:{:02}:{:02}\n{:?}\n",
                    _now.hour(),
                    _now.minute(),
                    _now.second(),
                    result_str
                )
                .expect("\nSomething wrong with writing message!\n");
            }
        }
        "c" | "console" => {
            println!("Listening...");
            info!("Listening...");

            loop {
                let _now = Utc::now();

                match socket.recv_from(&mut buf) {
                    Ok((number_of_bytes, src_addr)) => {
                        println!("received bytes: {:?} from {:?}", buf, src_addr);
                        result = Vec::from(&buf[0..number_of_bytes]);
                        while result.last() == Some(&10) || result.last() == Some(&0) {
                            result.pop();
                        }
                    }
                    Err(fail) => println!("failed listening {:?}", fail),
                }
                let display_result = result.clone();
                let result_str = String::from_utf8(display_result).unwrap();
                println!(
                    "received message - time : {:02}:{:02}:{:02}\n{:?}",
                    _now.hour(),
                    _now.minute(),
                    _now.second(),
                    result_str
                );
            }
        }
        _ => {
            println!("Something wrong at trying to start listen...");
            info!("Something wrong at trying to start listen...");
        }
    };
}

/// It creates a socket and binds it to the specified port.
///
/// Arguments:
///
/// * `port`: u32 - port number
///
/// Returns:
///
/// Result<std::net::UdpSocket, std::io::Error>
fn create_socket(port: u32) -> Result<std::net::UdpSocket, std::io::Error> {
    if 1024 > port && port > 65535 {
        panic!("Port must be a range from 1024 to 65535!");
    } else {
        let addr = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port.try_into().unwrap(),
        );
        let socket = UdpSocket::bind(addr);

        match socket {
            Ok(socket) => Ok(socket),
            Err(error) => panic!("[------------------------------------]\nERROR!: {}\n[------------------------------------]", error)
        }
    }
}

/// It removes a given string from a vector of strings
///
/// Arguments:
///
/// * `args`: &'a mut Vec<&str> - this is the vector that we're going to remove the item from.
/// * `x1`: &str - the string to be removed from the vector
///
/// Returns:
///
/// A vector of strings.
fn remove_item_from_vec<'a>(args: &'a mut Vec<&str>, x1: &str) -> Vec<&'a str> {
    info!("Removing bin path from env variables...");
    let mut i: usize = 0;
    while i < args.len() {
        if args[i].contains(x1) == true && i < args.len() {
            println!("Checking {} argument...\nRemoving...\nDone!", i + 1);
            args.remove(i);
            i += 1;
            continue;
        } else {
            /*
            let index = args.iter().position(|x| *x == x1).unwrap();
            args.remove(i);
            */
            i += 1;
            println!("Checking {} argument...\nOK!", i);
            continue;
        }
    }
    info!("Removing done!");
    args.to_vec()
}

/// It takes a vector of strings, checks if it's empty, if it's not, it checks if it has one argument,
/// if it has, it checks if it's a port, if it's not, it checks if it has two arguments, if it has, it
/// checks if it's a port and a mode, if it's not, it checks if it has three arguments, if it has, it
/// checks if it's a port, a mode and a file, if it's not, it checks if it has four arguments, if it
/// has, it checks if it's a port, a mode and a file, if it's not, it checks if it has more than four
/// arguments, if it has, it prints an error message and panics, if it doesn't, it prints an error
/// message and panics.
/// 
/// Arguments:
/// 
/// * `argsuments_in_str`: Vec<&str> - a vector of arguments in the form of a string
fn get_parametrs(mut argsuments_in_str: Vec<&str>) -> (UdpSocket, &str) {
    println!(
        "Got {:?} arguments: {:?}",
        argsuments_in_str.len(),
        &argsuments_in_str[..]
    );
    info!(
        "Got {:?} arguments: {:?}",
        argsuments_in_str.len(),
        &argsuments_in_str[..]
    );

    let args: Vec<&str> = remove_item_from_vec(&mut argsuments_in_str, "rusty_listener");

    println!(
        "Got {:?} arguments after cleaning: {:?}",
        args.len(),
        &args[..]
    );
    info!(
        "Got {:?} arguments after cleaning: {:?}",
        args.len(),
        &args[..]
    );

    let params: (UdpSocket, &str) = match args.len() {
        0 => {
            println!("Got no arguments, setting up default parametrs. Console mode, port 5054");
            info!("Got no arguments, setting up default parametrs. Console mode, port 5054");
            (create_socket(5054).unwrap(), "c")
        }
        1 => {
            println!("Got no arguments, setting up default parametrs. Console mode, port 5054");
            error!("Not enought arguments! Can't work in that way! Setting up default parametrs");
            (create_socket(5054).unwrap(), "c")
        }
        2 => {
            //
            if (args[0] == "-m" || args[0] == "--mode") && (args[1] == "c" || args[1] == "console")
            {
                println!(
                    "Setting up {} mode. Parametrs: Console mode, port 5054",
                    args[1]
                );
                info!(
                    "Setting up {} mode. Parametrs: Console mode, port 5054",
                    args[1]
                );
                (create_socket(5054).unwrap(), "c")
            }
            //
            else if (args[0] == "-m" || args[0] == "--mode")
                && (args[1] == "f" || args[1] == "file")
            {
                println!(
                    "Setting up {} mode. Parametrs: File mode, port 5054",
                    args[1]
                );
                info!(
                    "Setting up {} mode. Parametrs: File mode, port 5054",
                    args[1]
                );
                (create_socket(5054).unwrap(), "f")
            }
            //
            else if (args[0] == "-p" || args[0] == "--port")
                && (args[1].parse::<u32>().unwrap() > 1024
                    && args[1].parse::<u32>().unwrap() < 65535)
            {
                (create_socket(args[1].parse::<u32>().unwrap()).unwrap(), "c")
            }
            //
            else if (args[0] == "-p" || args[0] == "--port")
                && (args[1].parse::<u32>().unwrap() < 1024
                    || args[1].parse::<u32>().unwrap() > 65535)
            {
                println!(
                    "Error! Bad port. Setting up console mode. Parametrs: Console mode, port 5054"
                );
                info!(
                    "Error! Bad port. Setting up console mode. Parametrs: Console mode, port 5054"
                );
                (create_socket(5054).unwrap(), "c")
            } else {
                error!("Wrong arguments! Setting default values. Mode: console, Port: 5054");
                (create_socket(5054).unwrap(), "c")
            }
        }
        3 => {
            error!("Wrong arguments! Setting default values. Mode: console, Port: 5054");
            (create_socket(5054).unwrap(), "c")
        }
        4 => {
            //1
            if (args[0] == "-m" || args[0] == "--mode")
                && (args[1] == "c" || args[1] == "console")
                && (args[2] == "-p" || args[2] == "--port")
                && (args[3].parse::<u32>().unwrap() > 1024
                    && args[3].parse::<u32>().unwrap() < 65535)
            {
                println!("hello");
                println!(
                    "Setting up console mode. Parametrs: {} mode, port {}",
                    args[1],
                    args[3].parse::<u32>().unwrap()
                );
                info!(
                    "Setting up console mode. Parametrs: {} mode, port {}",
                    args[1],
                    args[3].parse::<u32>().unwrap()
                );
                (create_socket(args[3].parse::<u32>().unwrap()).unwrap(), "c")
            }
            //2
            else if (args[0] == "-m" || args[0] == "--mode")
                && (args[1] == "f" || args[1] == "file")
                && (args[2] == "-p" || args[2] == "--port")
                && (args[3].parse::<u32>().unwrap() > 1024
                    && args[3].parse::<u32>().unwrap() < 65535)
            {
                println!(
                    "Setting up file mode. Parametrs: {} mode, port {}",
                    args[1],
                    args[3].parse::<u32>().unwrap()
                );
                info!(
                    "Setting up file mode. Parametrs: {} mode, port {}",
                    args[1],
                    args[3].parse::<u32>().unwrap()
                );
                (create_socket(args[3].parse::<u32>().unwrap()).unwrap(), "f")
            }
            //3
            else if (args[0] == "-p" || args[0] == "--port")
                && (args[1].parse::<u32>().unwrap() > 1024
                    && args[1].parse::<u32>().unwrap() < 65535)
                && (args[2] == "-m" || args[2] == "--mode")
                && (args[3] == "c" || args[3] == "console")
            {
                println!(
                    "Setting up {} mode. Parametrs: {} mode, port {}",
                    args[0],
                    args[1],
                    args[1].parse::<u32>().unwrap()
                );
                info!(
                    "Setting up {} mode. Parametrs: {} mode, port {}",
                    args[0],
                    args[1],
                    args[1].parse::<u32>().unwrap()
                );
                (create_socket(args[1].parse::<u32>().unwrap()).unwrap(), "c")
            }
            //4
            else if (args[0] == "-p" || args[0] == "--port")
                && (args[1].parse::<u32>().unwrap() > 1024
                    && args[1].parse::<u32>().unwrap() < 65535)
                && (args[2] == "-m" || args[2] == "--mode")
                && (args[3] == "f" || args[3] == "file")
            {
                println!(
                    "Setting up {} mode. Parametrs: {} mode, port {}",
                    args[0],
                    args[1],
                    args[1].parse::<u32>().unwrap()
                );
                info!(
                    "Setting up {} mode. Parametrs: {} mode, port {}",
                    args[0],
                    args[1],
                    args[1].parse::<u32>().unwrap()
                );
                (create_socket(args[3].parse::<u32>().unwrap()).unwrap(), "f")
            }
            //5
            else if (args[0] == "-m" || args[0] == "--mode")
                && (args[1] == "c" || args[1] == "console")
                && (args[2] == "-p" || args[2] == "--port")
                && (args[3].parse::<u32>().unwrap() < 1024
                    && args[3].parse::<u32>().unwrap() > 65535)
            {
                println!(
                    "Bad port. Setting up {} mode. Parametrs: {} mode, port 5054",
                    args[1], args[1]
                );
                info!(
                    "Bad port. Setting up {} mode. Parametrs: {} mode, port 5054",
                    args[1], args[1]
                );
                (create_socket(5054).unwrap(), "c")
            }
            //6
            else if (args[0] == "-m" || args[0] == "--mode")
                && (args[1] == "f" || args[1] == "file")
                && (args[2] == "-p" || args[2] == "--port")
                && (args[3].parse::<u32>().unwrap() < 1024
                    && args[3].parse::<u32>().unwrap() > 65535)
            {
                println!(
                    "Bad port. Setting up file mode. Parametrs: {} mode, port 5054",
                    args[1]
                );
                info!(
                    "Bad port. Setting up file mode. Parametrs: {} mode, port 5054",
                    args[1]
                );
                (create_socket(5054).unwrap(), "f")
            }
            //7
            else if (args[0] == "-m" || args[0] == "--mode")
                && (args[1] != "f" || args[1] != "file" && args[1] != "c" || args[1] != "console")
                && (args[2] == "-p" || args[2] == "--port")
                && (args[3].parse::<u32>().unwrap() > 1024
                    && args[3].parse::<u32>().unwrap() < 65535)
            {
                println!(
                    "Bad mode param. Parametrs: {} mode, port {}",
                    args[1],
                    args[3].parse::<u32>().unwrap()
                );
                info!(
                    "Bad mode param. Parametrs: {} mode, port {}",
                    args[1],
                    args[3].parse::<u32>().unwrap()
                );
                (create_socket(args[3].parse::<u32>().unwrap()).unwrap(), "c")
            }
            //8
            else {
                error!("Wrong arguments! Setting default values. Mode: console, Port: 5054");
                (create_socket(5054).unwrap(), "c")
            }
        }
        _ => {
            println!("\n\n\n\n\nError!\nExecute program with arguments than match the port and mode. If none will be matched, arguments set by default \nExample -> rusty -m or --mode f -p or --port 5054\n\n\n\n");
            panic!("arguments checking stage");
        }
    };
    info!("Checking arguments done!");
    println!("Checking arguments done!");
    info!("Now socket is - {:?}, mode is - {}", params.0, params.1);
    println!("Now socket is - {:?}, mode is - {}", params.0, params.1);
    params.into()
}

/// The main function of the program.
/// 
/// Returns:
/// 
/// A tuple of UdpSocket and &str
fn main() -> Result<(), Box<dyn std::error::Error>> {
    current_os();
    init_cli_log!("rusty");

    info!("Initiating variables...");
    let arguments_in_string: Vec<String> = env::args().collect();
    let argsuments_in_str: Vec<&str> = arguments_in_string.iter().map(AsRef::as_ref).collect();
    info!("Initiating variables done!");

    println!("Trying to start listen...");
    info!("Trying to start listen...");

    let all: (UdpSocket, &str) = get_parametrs(argsuments_in_str);
    start_listening(all);

    info!("Listening done...");

    Ok(())
}

//region shit
// struct Arguments{
//     mode: &str,
//     mode_value: &str,
//     port: &str,
//     port_value: &str
// }
// impl Arguments{
//     fn parse_arguments_from_vec(&mut self, argsuments_in_str: Vec<&str>)
//     {

//         let mut port_exist: bool = false;
//         let mut mode_exist: bool = false;
//         // self.s = argsuments_in_str.into_iter().collect();
//         // self.mode = argsuments_in_str[..].find("-m").expect("not found argumet that set mode");
//         // self.mode_value = argsuments_in_str[self.mode + 1].expect("mode value was not found");
//         // self.port = argsuments_in_str.find("-r").expect("not found argumet that set port");
//         // self.port_value = argsuments_in_str[self.port + 1].expect("port value was not found");
//     }
//     fn display(&self)
//     {
//         println!("port: {}, mode: {}", self.port_value, self.mode_value);
//         info!("port: {}, mode: {}", self.port_value, self.mode_value);
//     }
// }

// // let mut args = Arguments{
// //     mode: 0,
// //     mode_value: " ".to_string(),
// //     port: 0,
// //     port_value: 0
// // };
// let argues = argsuments_in_str.clone();
// args.parse_arguments_from_vec(argues);
// args.display();

//endregion