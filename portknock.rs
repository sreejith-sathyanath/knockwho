use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::collections::HashMap;

// Define the closed ports to listen on for the port knocking sequence.
const CLOSED_PORTS: [u16; 3] = [10000, 10001, 10002];
// Define the specific port to open if the sequence is successful.
const OPEN_PORT: u16 = 10003;
// Define the password as a sequence of IP addresses and timestamps.
const PASSWORD: [(String, u64); 3] = [
    ("192.168.0.1", 1500000000),
    ("192.168.0.2", 1500000001),
    ("192.168.0.3", 1500000002),
];

// Create a buffer to store the sequence of connection attempts.
let mut buffer: HashMap<String, u64> = HashMap::new();

// Listen on each of the closed ports for incoming connection attempts.
for port in CLOSED_PORTS.iter() {
    // Create a TcpListener to listen on the current port.
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();

    // Accept incoming connections and add their IP addresses and timestamps to the buffer.
    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                let peer_addr = s.peer_addr().unwrap();
                let timestamp = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                buffer.insert(peer_addr.ip().to_string(), timestamp);
                s.shutdown(Shutdown::Both).unwrap();
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}

// Compare the sequence of connection attempts in the buffer with the password.
let mut correct_sequence = true;
for (i, (ip, timestamp)) in PASSWORD.iter().enumerate() {
    if buffer[ip] != timestamp {
        correct_sequence = false;
        break;
    }
}

// If the sequence is correct, open the specific port.
if correct_sequence {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", OPEN_PORT)).unwrap();

    // Accept incoming connections on the open port.
    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                // Handle the connection.
                let mut data = [0; 1024];
                match s.read(&mut data) {
                    Ok(_) => {
                        println!("Received data: {}", String::from_utf8_lossy(&data));
                        s.write(&data).unwrap();
                    }
                    Err(e) => println!("Error: {}", e),
                }

                s.shutdown(Shutdown::Both).unwrap();
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
