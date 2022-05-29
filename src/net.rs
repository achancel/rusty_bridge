pub mod net_io {
//crate and imports
    use log::{info};
    use chrono::{Timelike, Utc};
    use std::fs::File;
    use std::io::ErrorKind;
    use std::io::Write;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};//
//functions

    /// It's a function that listens to the socket and prints the received message to the console or writes
    /// it to the file
    /// 
    /// Arguments:
    /// 
    /// * `socket`: UdpSocket - the socket we're going to listen on
    /// * `mode`: &str - mode of outputting accepted info.
    pub fn listen(socket: UdpSocket, mode: &str) {
        
        info!("Enter at listen fn! ✓");

        let mut buf = vec![0; 1024];
        let mut result: Vec<u8> = Vec::new();      

        match mode {

            "f" | "file" => {

                let now = Utc::now();

                //read from socket
                    match socket.recv_from(&mut buf) {
                    //good case
                        Ok((number_of_bytes, src_addr)) => {
                            //print recieved info
                                println!("received bytes: {:?} from {:?}", number_of_bytes, src_addr);
                                result = Vec::from(&buf[0..number_of_bytes]);//

                            //removing line crossing char
                                while result.last() == Some(&10) || result.last() == Some(&0) {
                                    result.pop();
                                }//

                        }//
                    //error case
                        Err(fail) => println!("failed listening {:?}", fail),//
                    }//

                //formating from bytes to utf chars
                    let display_result = result.clone();
                    let result_str = String::from_utf8(display_result).unwrap();//

                //writing to file accepted info
                    //file error handling block
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
                        };//
                    write!(
                        file,
                        "Received message - time : {:02}:{:02}:{:02}\n{:?}\n", now.hour(), now.minute(), now.second(), result_str)
                        .expect("\nSomething wrong with writing message!\n");//
            }

            "c" | "console" => { 

                let now = Utc::now();

                //read from socket
                    match socket.recv_from(&mut buf) {
                    //good case
                        Ok((number_of_bytes, src_addr)) => {
                            //print recieved info
                                println!("received bytes: {:?} from {:?}", number_of_bytes, src_addr);
                                result = Vec::from(&buf[0..number_of_bytes]);//

                            //removing line crossing char
                                while result.last() == Some(&10) || result.last() == Some(&0) {
                                    result.pop();
                                }//

                        }//
                    //error case
                            Err(fail) => println!("failed listening {:?}", fail),//
                            }//

                //formating from bytes to utf chars
                //outputting accepted info
                    let display_result = result.clone();
                    let result_str = String::from_utf8(display_result).unwrap();//

                
                    println!(
                    "Received message - time : {:02}:{:02}:{:02}\n{:?}",
                    now.hour(),
                    now.minute(),
                    now.second(),
                    result_str
                    );//
            }
            _ => {
                println!("Something wrong at trying to start listen...");
                info!("Something wrong at trying to start listen...");
            }
        };
    }

    /// It takes a socket and a receiver as arguments, reads a message from the user, converts it to bytes,
    /// and sends it to the receiver
    /// 
    /// Arguments:
    /// 
    /// * `socket`: UdpSocket,
    /// * `receiver`: String - The receiver's IP address.
    pub fn forward(socket: UdpSocket, receiver: String){
        
        info!("Enter at send fn! ✓");

        let now = chrono::Local::now();

        let mut message = String::new();
        std::io::stdin().read_line(&mut message).unwrap();
        let msg_bytes = message.into_bytes();

        info!("Input accepted! ✓");


        println!("Sending data - time : {:}:{:02}:{:02} -----------------------^\n", now.hour(), now.minute(), now.second());//  .hour(), now.minute(), now.second());

        socket.send_to(&msg_bytes, receiver).expect("failed to send message");

        info!("Message sended! ✓");

    }

    /// > This function creates a socket and binds it to the given port
    /// 
    /// Arguments:
    /// 
    /// * `port`: The port you want to bind the socket to.
    /// 
    /// Returns:
    /// 
    /// A Result<std::net::UdpSocket, std::io::Error>
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
}//