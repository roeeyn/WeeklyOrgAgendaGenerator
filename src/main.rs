// use std::time::SystemTime;
use chrono;
use chrono::offset::Local;
use chrono::DateTime;
use chrono::Duration;
use chrono::TimeZone;
use chrono::{Date, Datelike};
use regex::Regex;
use std::env;
use std::string::String;

// fn get_week_number(today_timestamp: DateTime<Local>) -> u32 {
fn get_week_number(today_timestamp: Date<Local>) -> u32 {
    // let naive_date = today_timestamp.naive_local().date();
    let naive_date = today_timestamp.naive_local();
    return naive_date.iso_week().week();
}

fn get_nice_date(today_timestamp: &Date<Local>) -> String {
    let naive_date = today_timestamp.naive_local();
    return naive_date.format("%A %d/%m/%y").to_string();
}

// fn get_scheduled_date(today_timestamp: DateTime<Local>) -> String {
fn get_scheduled_date(today_timestamp: &Date<Local>) -> String {
    // let naive_date = today_timestamp.naive_local().date();
    let naive_date = today_timestamp.naive_local();
    return naive_date.format("%Y-%m-%d %a").to_string();
}

fn is_valid_arg(args: &Vec<String>) -> bool {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    if args.len() < 2 {
        return false;
    }
    if !re.is_match(&args[1]) {
        return false;
    }

    return true;
}

fn get_dmy(arg_date: &String) -> (u32, u32, i32) {

    let numbers: Vec<&str> = arg_date.split("-").collect();
    let day: u32 = numbers[2].parse().unwrap();
    let month: u32 = numbers[1].parse().unwrap();
    let year: i32 = numbers[0].parse().unwrap();

    return (day, month, year);
}

fn create_agenda(day:u32, month: u32, year: i32)-> std::string::String {
    let today = Local.ymd(year, month, day);
    let mut result_string = String::new();
    result_string.push_str(&format!("** Week {}\n",get_week_number(today)));
    
    // 0 -> Monday, 6 -> Sunday
    for i in 0..7 {
        println!("{}", i);
        let not_today = today + Duration::days(i);
        result_string.push_str(&format!("*** {}\n", get_nice_date(&not_today)));
        result_string.push_str("**** TODO COSA\n");
        result_string.push_str(&format!("     SCHEDULED: <{} horario>\n", get_scheduled_date(&not_today)));
        result_string.push_str("     :PROPERTIES:\n");
        result_string.push_str("       :WILD_NOTIFIER_NOTIFY_BEFORE: 5 0\n");
        result_string.push_str("     :END:\n\n");

    }


    // println!("{:?}", get_week_number(today));
    // println!("{:?}", get_nice_date(today));
    // println!("{:?}", get_scheduled_date(today));
    return result_string;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if !is_valid_arg(&args) {
        eprintln!("You have to put the date of the Monday as an argument e.g. yyyy-mm-dd");
        return;
    }

    let result = match get_dmy(&args[1]) {
        (day, month, year) => create_agenda(day, month, year),
    };

    println!("{}", result);


}
