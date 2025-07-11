# Rust Port Sniffer CLI

A lightweight command-line port sniffer written in Rust, designed to scan IP addresses for open ports with configurable threading.

## What is a Port Sniffer?

A port sniffer (or port scanner) is a tool that probes a target IP address or host to determine which network ports are open, closed, or filtered. This is useful for network diagnostics, security auditing, and service discovery.

## Features

- Multi-threaded scanning for faster results
- Simple CLI interface with help guide
- Basic error handling for invalid inputs

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/rust-port-sniffer.git
cd ip_sniffer

# Build the project (debug mode)
cargo build

# For production use (optimized)
cargo build --release
```

## Usage

```bash
# Display help screen
ip_sniffer.exe -h

# Scan an IP with default threads (4)
ip_sniffer.exe 192.160.1.1

# Scan with custom thread count (e.g., 100 threads)
ip_sniffer.exe -j 100 192.160.1.1
```

### Important Notes About Arguments

1. The executable expects arguments in this format:
   ```bash
   cargo run -- [arguments]  # Note the required '--' separator
   ```

2. Argument structure when running the built binary:
   ```bash
   ip_sniffer.exe [FLAG] [THREADS] [IP_ADDRESS]
   ```

3. Argument requirements:
   - Minimum 2 arguments (just IP address)
   - Maximum 4 arguments (including thread flag and count)
   - Example argument breakdown:
     ```bash
     $ cargo run -- -j 10 192.160.1.1
     Output representation:
     target\debug\ip_sniffer.exe  // Program name
     -j                          // Thread flag
     10                          // Thread count
     192.160.1.1                 // Target IP
     ```

## Error Handling

The program will return errors for:
- Insufficient arguments (< 2)
- Too many arguments (> 4)
- Invalid IP address format
- Invalid thread count (non-numeric or unsafe values)

## Building

```bash
cargo build --release
```

The compiled binary will be available at `target/release/ip_sniffer.exe` (Windows) or `target/release/ip_sniffer` (Unix-like systems).