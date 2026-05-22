use now_time::{TimeSnapshot, now_local, now_utc, unix_timestamp, unix_timestamp_millis};

fn main() {
    // -- convenience free functions (fastest path) --
    println!("=== quick functions ===");
    println!("utc now      : {}", now_utc());
    println!("local now    : {}", now_local());
    println!("unix seconds : {}", unix_timestamp());
    println!("unix millis  : {}", unix_timestamp_millis());

    // -- full TimeSnapshot API --
    println!("\n=== TimeSnapshot::now_utc() ===");
    let t = TimeSnapshot::now_utc();
    println!("rfc3339      : {}", t.to_rfc3339());
    println!("rfc2822      : {}", t.to_rfc2822());
    println!("unix secs    : {}", t.unix_seconds());
    println!("unix millis  : {}", t.unix_millis());

    // -- custom format --
    println!("\n=== custom format ===");
    let date = t.format("%Y-%m-%d").expect("date format is valid");
    let time = t.format("%H:%M:%S").expect("time format is valid");
    let full = t.format("%A, %B %e %Y at %H:%M UTC").expect("full format is valid");
    println!("date         : {date}");
    println!("time         : {time}");
    println!("full         : {full}");

    // -- local time with offset --
    println!("\n=== local time ===");
    let local = TimeSnapshot::now_local();
    println!("local rfc3339: {}", local.to_local_rfc3339());
}