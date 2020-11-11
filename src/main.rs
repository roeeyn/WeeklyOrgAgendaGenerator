// use std::time::SystemTime;
use chrono;
use chrono::DateTime;
use chrono::Duration;
use chrono::offset::Local;
use chrono::{Datelike, Date};
use std::string::String;
use chrono::TimeZone;

// fn get_week_number(today_timestamp: DateTime<Local>) -> u32 {
fn get_week_number(today_timestamp: Date<Local>) -> u32 {
    // let naive_date = today_timestamp.naive_local().date();
    let naive_date = today_timestamp.naive_local();
    return naive_date.iso_week().week();
}

fn get_nice_date(today_timestamp: DateTime<Local>) -> String {
    let naive_date = today_timestamp.naive_local().date();
    return naive_date.format("%d/%m/%y").to_string();
}

// fn get_scheduled_date(today_timestamp: DateTime<Local>) -> String {
fn get_scheduled_date(today_timestamp: Date<Local>) -> String {
    // let naive_date = today_timestamp.naive_local().date();
    let naive_date = today_timestamp.naive_local();
    return naive_date.format("%Y-%m-%d %a").to_string();
}

fn main() {
    // let today = chrono::offset::Local::now();
    // let today = Local.ymd(2020, 11, 10).and_hms_milli(9, 10, 11, 12);
    let today = Local.ymd(2020, 11, 10);
    
    // println!("{:?}", get_week_number(today));
    // println!("{:?}", get_nice_date(today));
    println!("{:?}", get_scheduled_date(today));

    let not_today = today + Duration::days(1);
    println!("{:?}", get_scheduled_date(not_today));
}
