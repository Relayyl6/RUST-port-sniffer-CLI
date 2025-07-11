use std::env; //allow us to pull argument out of CLI
use std::net::{IpAddr, TcpStream}; // the IpAddr enum // TcpStream to connect to our Ip address
use std::str::FromStr; // allows us to convert our string to our ip address type
use std::process; // allows us to manage the way that our program shuts down
use std::sync::mpsc::{Sender, channel}; // allows us to send messages between threads, in this case, the main thread and the worker threads
use std::thread; // allows us to create threads
use std::io::{self, Write};
use std::time::Duration;

const MAX_PORT_TO_SNIFF: u16 = 1000;

#[derive(Debug)]
struct Argument {
    flag: String,
    threads: u16,
    ipaddr: IpAddr,
} // i think struct is like OOP

impl Argument { // creating an implementation block that creates methods that allows us to instantiate the Argument struct
    fn new(args: &[String]) -> Result<Argument, &'static str> { // takes an argument of type, a reference to a vector of string, and returns a Result, with Argument for Ok portion and a static reference to slice of string in the error portion // note: read up on the static lifetime
        if args.len() < 2 {
            return Err("Not Enough Arguments");
        } else if args.len() > 4 {
            return Err("Too Many Arguments");
        }
        let f = args[1].clone(); // should this be at index 3? i think // OK! dont worry, it only goes for cases like this "ip_sniffer.exe 192.160.1.1" any other case is "wrong" so they go to the ese block 
        if let Ok(ipaddr) = IpAddr::from_str(&f) { // IpAddr returns either an Ok or Err if the passed arg matches one of the types, V4 or V6, we're seeing, it then assigns ipaddr as this value 
            return Ok(Argument {
                flag: String::from(""), // empty string
                threads: 4, // default thread number
                ipaddr,
            }) // instantiate an instance of the struct
        } else { // handling the Err if wwe dont get an Ok // i.e. wither a -j or -h or -help was passed as the second elemtn in the vector
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 { // format: ip_sniffer.exe -h, so yes args.len() == 2 is valid
                println!("Usage: -j to select how many threads you want
                \r\n      -h or -help to show this help message");
                return Err("help")
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("Too Many Arguments");
            } else if flag.contains("-j") {
                if args.len() != 4 {
                    return Err("Invalid number of arguments for -j flag");
                }
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(ip) => ip,
                    Err(_) => return Err("Not a valid IP ADDRESS; must be IPv4 or IPv6")
                };
                let threads = match args[2].parse::<u16>(){
                    Ok(thread) => thread,
                    Err(_) => return Err("Faield to parse thread number")
                };
                return Ok(Argument {
                    flag,
                    threads,
                    ipaddr,
                }) // all of them used the short init instead of flag: flag, we do flag,
            } else {
                return Err("Invalid syntax");
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        if port == 0 {
            port += num_threads;
            continue;
        }

        match TcpStream::connect_timeout(&(addr, port).into(), Duration::from_millis(100)) {
            Ok(_) => { // signifies the port is open
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap(); // Each thread sends a value
            }
            Err(_) => {}
        }
        // Safely increment port, checking for overflow
        if port > MAX_PORT_TO_SNIFF - num_threads {
            break;
        }

        port += num_threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone(); // to get the program name
    
    // we could do this, to hold our data in the args but its too verbose // so instead we'll define and add all the information into a struct and implement it with the new method
    // let flag = args[1].clone()
    // let threads = args[2].clone()
    // let ipaddr = args[3].clone()

    let argument = Argument::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0) // exit with success
            } else {
                eprintln!("{} Problem parsing arguments: {}", program, err); // error print new line
                process::exit(1);
            }
        }
    );

    println!("Parsed arguments: {:?}", argument);

    let num_threads = argument.threads; // bind the number of threads to a variable
    let addr = argument.ipaddr;
    let _ = argument.flag;
    let mut handles = vec![];
    let ( tx, rx ) = channel(); // a tuple gets returned from channel, so we're destructuring it to get a transmitter and receiver

    println!("Starting scan on {} with {} threads...", addr, num_threads);

    for i in 0..num_threads { // iterate from 0 to the number of threads
        let tx = tx.clone(); // each thread has its own transmitter

        let handle = thread::spawn(move || { //  the move keyword is like making a photocopy(taking ownership) of th epassed arguments before sending data to a thread.
            scan(tx, i, addr, num_threads);
        }); // spawning our thread with the move closure

        handles.push(handle);
    }

    drop(tx); // Drop the original transmitter // Drop the original transmitter before waiting for threads

    println!("Waiting for threads to complete...");

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Scan completed, collecting results...");

    let mut out = vec![];
    for received in rx { // Collect messages from the receiver
        out.push(received);
    }

    println!(" ");
    if out.is_empty() {
        println!("No open ports found")
    } else {
        out.sort(); // Sort the collected port numbers
        for port_number in out {
            println!("{} is open", port_number); // Print each open port
        }
    }
}
