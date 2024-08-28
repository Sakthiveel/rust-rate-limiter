use chrono::{DateTime, Utc};
#[derive(Debug)]
pub struct Record {
    limit: i32,
    cur_req_count: i32,
    first_req_time: DateTime<Utc>,
    last_req_time: DateTime<Utc>,
    reset_time: f64,
    interval: i64, // in seconds
}

impl Record {
    pub fn new(
        limit: i32,
        cur_req_count: i32,
        first_req_time: DateTime<Utc>,
        reset_time: f64, // in mins
        interval: i64,   // in secs
    ) -> Record {
        Record {
            limit,
            cur_req_count,
            first_req_time,
            last_req_time: first_req_time.clone(),
            reset_time,
            interval,
        }
    }
    pub fn get_cur_req_count(&self) -> i32 {
        return self.cur_req_count;
    }

    pub fn increment_cur_req_count(&mut self) {
        self.cur_req_count = self.cur_req_count + 1;
    }

    pub fn get_last_req_time(&self) -> DateTime<Utc> {
        self.last_req_time
    }

    pub fn update_last_req_time(&mut self) {
        self.last_req_time = Utc::now();
    }

    pub fn get_first_req_time(&mut self) -> DateTime<Utc> {
        self.first_req_time
    }

    pub fn get_reset_time(&mut self) -> f64 {
        self.reset_time
    }

    pub fn get_interval(&mut self) -> i64 {
        self.interval
    }
    pub fn get_limit(&mut self) -> i32 {
        self.limit
    }
}
