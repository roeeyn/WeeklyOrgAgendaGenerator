use chrono::offset::Local;
use chrono::{Date, Datelike, Duration, TimeZone};
use regex::Regex;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::path::Path;
use std::string::String;

#[derive(Debug, Deserialize)]
struct Task {
    name: String,
    schedule: String,
}

fn get_week_number(today_timestamp: Date<Local>) -> u32 {
    let naive_date = today_timestamp.naive_local();

    naive_date.iso_week().week()
}

fn get_nice_date(today_timestamp: &Date<Local>) -> String {
    let naive_date = today_timestamp.naive_local();

    naive_date.format("%A %d/%m/%y").to_string()
}

fn get_scheduled_date(today_timestamp: &Date<Local>) -> String {
    let naive_date = today_timestamp.naive_local();

    naive_date.format("%Y-%m-%d %a").to_string()
}

fn is_valid_arg(args: &Vec<String>) -> bool {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    if args.len() < 3 {
        return false;
    }
    if !re.is_match(&args[1]) {
        return false;
    }

    true
}

fn get_tasks_from_file(path: &String) -> std::vec::Vec<Task> {
    let json_file_path = Path::new(path);
    let file = File::open(json_file_path).expect("File not found");

    serde_json::from_reader(file).expect("error while reading")
}

fn get_dmy(arg_date: &String) -> (u32, u32, i32) {
    let numbers: Vec<&str> = arg_date.split("-").collect();
    let day: u32 = numbers[2].parse().unwrap();
    let month: u32 = numbers[1].parse().unwrap();
    let year: i32 = numbers[0].parse().unwrap();

    (day, month, year)
}

fn create_agenda(day: u32, month: u32, year: i32, tasks_file_path: &String) -> std::string::String {
    let today = Local.ymd(year, month, day);
    let mut result_string = String::new();
    result_string.push_str(&format!("** Week {}\n", get_week_number(today)));

    let tasks = get_tasks_from_file(tasks_file_path);

    // 0 -> Monday, 6 -> Sunday
    for i in 0..7 {
        let not_today = today + Duration::days(i);
        result_string.push_str(&format!("\n*** {}\n", get_nice_date(&not_today)));
        for task in &tasks {
            result_string.push_str(&format!(
                "
**** TODO {task_name}
     SCHEDULED: <{scheduled_date} {task_schedule}>
     :PROPERTIES:
       :WILD_NOTIFIER_NOTIFY_BEFORE: 5 0
     :END:
",
                task_name = task.name,
                scheduled_date = get_scheduled_date(&not_today),
                task_schedule = task.schedule
            ))
        }
    }

    result_string
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if !is_valid_arg(&args) {
        eprintln!("You have to put the date of the Monday as an argument e.g. yyyy-mm-dd");
        eprintln!("You have to put the correct file path e.g. /Users/roeeyn/tasks.json");
        return;
    }

    let result = match get_dmy(&args[1]) {
        (day, month, year) => create_agenda(day, month, year, &args[2]),
    };

    println!("{}", result);
}
