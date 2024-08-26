mod record;
use record::Record;
use std::collections::HashMap;

use chrono::{DateTime, TimeDelta, Utc};

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

    pub fn consume(&mut self, ip: String, _cur_time: DateTime<Utc>) -> bool {
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
fn main() {
    let mut rate_limiter: InMemoryStorage = InMemoryStorage::new(4, 15);
    while true {
        let res: bool = rate_limiter.consume(String::from("ip__address"), Utc::now());

        println!("Response = {:?}", res);

        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
