use chrono::{DateTime, Utc};
#[derive(Debug)]
pub struct Record {
    cur_req_count: i32,
    first_req_time: DateTime<Utc>,
}

impl Record {
    pub fn new(cur_req_count: i32, first_req_time: DateTime<Utc>) -> Record {
        Record {
            cur_req_count,
            first_req_time,
        }
    }
    pub fn get_cur_req_count(&self) -> i32 {
        return self.cur_req_count;
    }

    pub fn increment_cur_req_count(&mut self) {
        self.cur_req_count = self.cur_req_count + 1;
    }

    pub fn get_first_req_time(&self) -> DateTime<Utc> {
        return self.first_req_time;
    }
}
