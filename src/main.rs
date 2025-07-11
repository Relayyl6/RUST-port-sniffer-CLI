use std::env; //allow us to pull argument out of CLI
use std::net::IpAddr; // the IpAddr enum
use std::str::FromStr; // allows us to convert our string to our ip address type
use std::process; // allows us to manage the way that our program shuts down

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
        if let Ok(ipaddr) IpAddr::from_str(&f) { // IpAddr returns either an Ok or Err if the passed arg matches one of the types, V4 or V6, we're seeing, it then assigns ipaddr as this value 
            return Ok(Argument {
                flag: String::from(""), // empty string
                threads: 4, // default thread number
                ipaddr
            }) // instantiate an instance of the struct
        } else { // handling the Err if wwe dont get an Ok // i.e. wither a -j or -h or -help was passed as the second elemtn in the vector
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 { // format: ip_sniffer.exe -h, so yes args.len() == 2 is valid
                println!("Usage: -j to select how many threads you want
                \r\n      -h or -help to show this help message")
                return Err("help")
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("Too Many Arguments");
            } else if flag.contains("-j") {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone() // to get the program name

    // we could do this, to hold our data in the args but its too verbose // so instead we'll define and add all the information into a struct and implement it with the new method
    // let flag = args[1].clone()
    // let threads = args[2].clone()
    // let ipaddr = args[3].clone()

    let argument = Argument::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0) // exit with success
            } else {
                eprintln!("Problem parsing arguments: {}", err); // error print new line
                process::exit(1); // exit with failure
            }
        }
    );

}
