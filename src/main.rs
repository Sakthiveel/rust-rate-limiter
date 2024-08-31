mod record;
use record::Record;
use std::{collections::HashMap, io::Read};

use chrono::{DateTime, TimeDelta, Utc};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

#[derive(Debug)]
struct InMemoryStorage {
    // limit: i32,
    // interval: i64,
    memory: HashMap<String, Record>,
}
impl InMemoryStorage {
    pub fn new() -> InMemoryStorage {
        InMemoryStorage {
            // limit,
            // interval,
            memory: HashMap::new(),
        }
    }

    pub fn consume(&mut self, ip: String, limit: i32, reset_time: f64, interval: i64) -> bool {
        if !self.memory.contains_key(&ip) {
            self.memory.insert(
                ip.clone(),
                Record::new(limit, 1, Utc::now(), reset_time, interval),
            );
            return true;
        }

        let cur_record: &mut Record = self.memory.get_mut(&ip).unwrap();
        let time_delta: TimeDelta = Utc::now() - cur_record.get_last_req_time();
        let seconds_diff: i64 = time_delta.num_seconds();
        let mut should_allow = false;
        if seconds_diff >= cur_record.get_interval()
            && cur_record.get_cur_req_count() < cur_record.get_limit()
        {
            cur_record.increment_cur_req_count();
            cur_record.update_last_req_time();
            should_allow = true;
        }
        let time_delta: TimeDelta = Utc::now() - cur_record.get_first_req_time();
        let total_time_diff: f64 = time_delta.num_seconds() as f64;
        println!(
            "Diff = {} ,reset_time = {} ",
            total_time_diff,
            cur_record.get_reset_time() * 60.0
        );
        if total_time_diff >= cur_record.get_reset_time() * 60.0 {
            should_allow = false;
            if seconds_diff >= cur_record.get_interval() {
                self.memory.remove(&ip);
                self.memory.insert(
                    ip.clone(),
                    Record::new(limit, 1, Utc::now(), reset_time, interval),
                );
                should_allow = true;
            }
        }

        return should_allow;
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000")?;
    println!("Server running at {}", listener.local_addr()?);
    // accept connections and process them serially
    let mut rate_limiter = InMemoryStorage::new();
    for stream in listener.incoming() {
        let ip_addr = stream?.peer_addr()?.ip().to_string();
        let res = rate_limiter.consume(ip_addr, 4, 0.2, 3);
        if res {
            println!("Allow");
        } else {
            println!("Blocked");
        }
        // handle_client(stream?);
    }

    Ok(())
}
