pub mod parser {

    // import logging macros
    use log::{error, info};     

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
    pub fn remove_item_from_vec(vec: &mut Vec<String>, x1: &str) -> Vec<String> {
        info!("Removing bin path from env variables...");
        let mut i: usize = 0;
        while i < vec.len() {
            if vec[i].contains(x1) == true && i < vec.len() {

                println!("Checking {} argument...\nNot OK - removing...", i + 1);

                vec.remove(i);

                i += 1;
                continue;
            } else {
                i += 1;
                println!("Checking {} argument...\nOK!", i);
                continue;
            }
        }
        info!("Removing done!");
        vec.to_vec()
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
    /// * `argsuments_in_string`: Vec<&str> - a vector of arguments in the form of a string
    pub fn get_parametrs(argsuments_in_string: Vec<String>) -> (String, String) {
        println!(
            "Got {:?} arguments: {:?}",
            argsuments_in_string.len(),
            &argsuments_in_string[..]
        );
        info!(
            "Got {:?} arguments: {:?}",
            argsuments_in_string.len(),
            &argsuments_in_string[..]
        );

        let mut vec: Vec<String> = argsuments_in_string.clone();
        let args: Vec<String> = remove_item_from_vec(&mut vec, "rusty_listener");

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

        let params: (String, String) = match args.len() {
            0 => {
                println!("Got no arguments, setting up default parametrs. Console mode, port 5054");
                info!("Got no arguments, setting up default parametrs. Console mode, port 5054");
                ("5054".to_string(), "c".to_string())
            }
            1 => {
                println!("Got no arguments, setting up default parametrs. Console mode, port 5054");
                error!("Not enought arguments! Can't work in that way! Setting up default parametrs");
                ("5054".to_string(), "c".to_string())
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
                    ("5054".to_string(), "c".to_string())
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
                    ("5054".to_string(), "f".to_string())
                }
                //
                else if (args[0] == "-p" || args[0] == "--port")
                    && (args[1].parse::<u32>().unwrap() > 1024
                        && args[1].parse::<u32>().unwrap() < 65535)
                {
                    (args[1].clone(), "c".to_string())
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
                    ("5054".to_string(), "c".to_string())
                } else {
                    error!("Wrong arguments! Setting default values. Mode: console, Port: 5054");
                    ("5054".to_string(), "c".to_string())
                }
            }
            3 => {
                error!("Wrong arguments! Setting default values. Mode: console, Port: 5054");
                ("5054".to_string(), "c".to_string())
            }
            4 => {
                //1
                if (args[0] == "-m" || args[0] == "--mode")
                    && (args[1] == "c" || args[1] == "console")
                    && (args[2] == "-p" || args[2] == "--port")
                    && (args[3].parse::<u32>().unwrap() > 1024
                        && args[3].parse::<u32>().unwrap() < 65535)
                {
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
                    (args[3].clone(), "c".to_string())
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
                    (args[3].clone(), "f".to_string())
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
                    (args[1].clone(), "c".to_string())
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
                    (args[3].clone(), "f".to_string())
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
                    ("5054".to_string(), "c".to_string())
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
                    ("5054".to_string(), "f".to_string())
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
                    (args[3].clone(), "c".to_string())
                }
                //8
                else {
                    error!("Wrong arguments! Setting default values. Mode: console, Port: 5054");
                    ("5054".to_string(), "c".to_string())
                }
            }
            _ => {
                println!("\n\n\n\n\nError!\nExecute program with arguments than match the port and mode. If none will be matched, arguments set by default \nExample -> rusty -m or --mode f -p or --port 5054\n\n\n\n");
                panic!("arguments checking stage");
            }
        };
        info!("Checking arguments done!");
        println!("Checking arguments done!");
        info!("Now port is - {:?}, mode is - {}", params.0, params.1);
        println!("Now port is - {:?}, mode is - {}", params.0, params.1);
        params.into()
    }
}
