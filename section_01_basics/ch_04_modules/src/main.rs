mod greetings;
mod meetings;

// use greetings::evening;
// use greetings::morning;
// use greetings::{evening, morning};
// use greetings::*;
use greetings::evening::*;
use greetings::morning::*;

fn main() {
    println!("Hello, world!");

    println!("{}", "-".repeat(40));
    meetings::hello();
    meetings::goodbye();

    println!("{}", "-".repeat(40));
    // greetings::morning::morning();
    // greetings::evening::evening();
    // morning::good_morning();
    // evening::good_evening();
    good_morning();
    good_evening();
}
