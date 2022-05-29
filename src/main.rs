//OS configurating block
    //This function only gets compiled if the target OS is linux
    #[cfg(target_os = "linux")]
    fn current_os() {
        info!("Current OS = Linux");
    }
    // And this function only gets compiled if the target OS is *not* linux
    #[cfg(not(target_os = "linux"))]
    fn current_os() {
        info!("Current OS != Linux");
    }//

//crates and imports
    use cli_log::*; // import logging macros
    use log::info;
    use clap::*;

    use std::thread;

    mod net;
    use net::net_io::*;//

//argument parser block
    #[derive(Parser, Debug)]
    #[clap(author, version, about)]

    struct Args {
        ///Mode argument
        #[clap(short, long, default_value = "c")]
        mode: String,

        ///Port argument
        #[clap(short, long, default_value_t = 5054)]
        port: u32,

        ///Reciever argument(JUST STRING. NO IP CHECK!!!)
        #[clap(short, long)]
        receiver: String,
    }//

//main function
    fn main() -> Result<(), Box<dyn std::error::Error>> {

        info!("Initiating...");

            current_os();
            info!("Checked OS! ✓");

            init_cli_log!("rusty");
            info!("Initiated logger! ✓");

            let args = Args::parse();
            info!("Arguments parsed! ✓");

            let socket: std::net::UdpSocket = create_socket(args.port).unwrap();
            let sock = socket.try_clone().unwrap();
            info!("Sockets created! ✓");

        info!("Initiating variables done! ✓");

        info!("Trying to start listen...");

            let listener = thread::spawn(move || loop {
                listen(sock.try_clone().unwrap(), args.mode.as_str())
            });

        info!("Listener started at new thread! ✓\nThread info: {:?}✓", listener);

        info!("Trying to start sender...");

            // Get the stdin from the user, and put it in read_string
            let sender = thread::spawn(move || loop { net::net_io::forward(socket.try_clone().unwrap(), args.receiver.clone()) });

            info!("Sender started at new thread! ✓\nThread info: {:?}✓", sender);

            sender.join().unwrap();

        info!("Sender joined to main thread! ✓");

        info!("Program shutdown! ✓");
        Ok(())
    }//