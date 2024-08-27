mod record;
use record::Record;
use std::{collections::HashMap, io::Read};

use chrono::{DateTime, TimeDelta, Utc};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

#[derive(Debug)]
struct InMemoryStorage {
    limit: i32,
    interval: i64,
    memory: HashMap<String, Record>,
}
impl InMemoryStorage {
    pub fn new(limit: i32, interval: i64) -> InMemoryStorage {
        InMemoryStorage {
            limit,
            interval,
            memory: HashMap::new(),
        }
    }

    pub fn consume(&mut self, ip: String) -> bool {
        println!("IP addres :{}", ip);
        println!("memory : {:?}", self.memory);
        if !self.memory.contains_key(&ip) {
            self.memory.insert(ip.clone(), Record::new(1, Utc::now()));
            return true;
        }

        let cur_record: &mut Record = self.memory.get_mut(&ip).unwrap();
        let time_delta: TimeDelta = Utc::now() - cur_record.get_first_req_time();
        let seconds_diff: i64 = time_delta.num_seconds();
        println!("Time diff {:?}", seconds_diff);
        println!("Record:{:?}", cur_record);
        let mut should_allow = false;
        if cur_record.get_cur_req_count() < self.limit {
            cur_record.increment_cur_req_count();
            println!("After increment {:?}", cur_record);
            println!("*****************************************************************");
            should_allow = true;
        }
        if seconds_diff > self.interval {
            self.memory.remove(&ip);
            println!("IP address blocked");
            should_allow = false;
        }

        return should_allow;
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000")?;
    println!("Server running at {}", listener.local_addr()?);
    // accept connections and process them serially
    let mut rate_limiter = InMemoryStorage::new(3, 10);
    for stream in listener.incoming() {
        let ip_addr = stream?.peer_addr()?.ip().to_string();
        let res = rate_limiter.consume(ip_addr);
        if res {
            println!("Allow");
        } else {
            println!("Blocked");
        }
        // handle_client(stream?);
    }

    Ok(())
}

// fn handle_client(mut stream: TcpStream) {
//     let mut buffer = [0; 1024]; // Create a buffer to hold the incoming data
//     if let Ok(addr) = stream.peer_addr() {
//         println!("Client IP Address: {}", addr.ip());
//     } else {
//         println!("Could not get client IP address.");
//     }
//     match stream.read(&mut buffer) {
//         Ok(_) => {
//             // Convert the buffer to a string
//             let request = String::from_utf8_lossy(&buffer[..]);
//
//             // Split the request by lines and get the first line
//             let request_line = request.lines().next().unwrap_or("");
//
//             // Split the request line by spaces and extract the method
//             let method = request_line.split_whitespace().next().unwrap_or("");
//
//             // Print the method (GET, POST, etc.)
//             let splitStr: Vec<&str> = request_line.split(" ").collect();
//             // println!("Req : {:?}", request);
//
//             // println!("DATA {request}");
//             // Optionally, respond to the client
//             let response = "HTTP/1.1 200 OK\r\n\r\nHello, client!";
//             stream
//                 .write_all(response.as_bytes())
//                 .expect("Failed to write response");
//             stream.flush().expect("Failed to flush stream");
//         }
//         Err(e) => {
//             eprintln!("Failed to read from the stream: {}", e);
//         }
//     }
// }
