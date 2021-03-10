mod bnb;
mod atom;

use std::time::{Duration, Instant};
use std::thread::sleep;
use chrono::prelude::*;


fn main() {
    // let delay = Duration::from_secs(3);
    // let start = Instant::now();
    // sleep(delay);
    // let duration = start.elapsed();
    //
    // println!("Time elapsed: {:?}", duration);

   // February 26, 2021, 6:00 AM GMT
   let end_time = Utc.ymd(2021, 2,26).and_hms(6,0,0);

   println!("{:?}", end_time);

   let now = Utc::now();
}