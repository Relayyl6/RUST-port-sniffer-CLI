types of command that should be available on the CLI
ip_sniffer.exe -h // display a help screen
ip_sniffer.exe -j 100 192.160.1.1 //specify the number of threads that the process will use // here, args(used in main code) is = 4, so > 4 should throw an error
ip_sniffer.exe 192.160.1.1 // callnig the tool on an IP address using the default set number of threads // here, args(used in main code) is = 2, so < 2 should throw an error


on the CLI, we pass argument towards the executable and not cargo itself like this "cargo run -- -h", emphasis on --

for e.g.
    for i in &args {
        println!("{}", i)
    }

    println!("{:?}", args)

this will print this to the console if we run "cargo run -- -j 10 192.160.1.1"
```bash
    target\debug\ip_sniffer.exe  // the program name // program
    -j // the thread flag // flag
    10 // the thread number // threads
    192.160.1.1 // the ip address // ipaddr
    ["target\\debug\\ip_sniffer.exe", "-j", "10", "192.160.1.1"]
```