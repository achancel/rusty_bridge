pub mod listener {
                                //
    use log::{info};     //import logging macros
    use chrono::{Timelike, Utc};//

    use std::fs::File;
    use std::io::ErrorKind;
    use std::io::Write;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
    use std::time::{SystemTime, Duration};

    /// It creates a UDP socket, binds it to the address and port specified in the config file, and then
    /// starts listening for incoming messages
    /// 
    /// Arguments:
    /// 
    /// * `all`: (String, )
    pub fn listen(socket: UdpSocket, params: (String, String)) {
        
        println!("Starting");
        info!("Starting");

        let mut buf = vec![0; 10];
        let mut result: Vec<u8> = Vec::new();
        let _now = SystemTime::now();
        //let port: u32 = params.0.parse::<u32>().unwrap();
        let mode: &str = params.1.as_str();        
        //let socket: UdpSocket = create_socket(port).unwrap();

        match mode {
            "f" | "file" => {
                println!("Listening...");
                info!("Listening...");

                    let _now = Utc::now();
                    std::thread::sleep(Duration::from_secs(5));

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

                    let mut file = match std::fs::OpenOptions::new().write(true).append(true).open("messages.txt"){
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
            "c" | "console" => {
                println!("Listening...");
                info!("Listening...");

                    let _now = Utc::now();
                    
                    println!("\n\n");
                    std::thread::sleep(Duration::from_millis(5));
                    println!("\n\n");

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
    pub fn create_socket(port: u32) -> Result<std::net::UdpSocket, std::io::Error> {
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
    
    /// The function takes a socket, a receiver, and a message, and sends the message to the receiver
    /// 
    /// Arguments:
    /// 
    /// * `socket`: The socket we're sending the message on
    /// * `receiver`: The IP address of the receiver.
    /// * `msg`: The message to be sent.
    fn send(socket: UdpSocket, receiver: String, msg: &Vec<u8>){

        println!("sending data");
        let result = socket.send_to(msg, receiver).expect("failed to send message");

    }
      
    //   fn main() {
    //     let client_arg = env::args().nth(1).unwrap();
    //     let mut buf = vec![0; 100];
    //     let socket = init_host();
    //     let message = String::from("hello from underground blyat");
    //     let msg_bytes = message.into_bytes();
      
    //     loop {
    //         send(&socket, &client_arg, &msg_bytes);
    //     }
    //  }
    
}

