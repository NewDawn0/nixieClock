// Setupbul
use pad::{PadStr, Alignment};
use chrono::Local;
use colored::Colorize;
use std::process::{exit, Command};
use simple_scheduler::{Duration, Schedule, ScheduleAt, ScheduledTask, task};
// Constants
const DIGITS : [[&str; 11]; 7] = [
    ["┏━━━┓","  ╻  ","┏━━━┓", "┏━━━┓","╻   ╻","┏━━━┓","┏━━━┓","┏━━━┓","┏━━━┓","┏━━━┓","     "],
    ["┃   ┃","  ┃  ","    ┃", "    ┃","┃   ┃","┃    ","┃    ","    ┃","┃   ┃","┃   ┃","  ╻  "],
    ["┃   ┃","  ┃  ","    ┃", "    ┃","┃   ┃","┃    ","┃    ","    ┃","┃   ┃","┃   ┃","     "],
    ["┃   ┃","  ┃  ","┏━━━┛", "┣━━━┫","┗━━━┫","┗━━━┓","┣━━━┓","    ┃","┣━━━┫","┗━━━┫","     "],
    ["┃   ┃","  ┃  ","┃    ", "    ┃","    ┃","    ┃","┃   ┃","    ┃","┃   ┃","    ┃","     "],
    ["┃   ┃","  ┃  ","┃    ", "    ┃","    ┃","    ┃","┃   ┃","    ┃","┃   ┃","    ┃","  ╹  "],
    ["┗━━━┛","  ╹  ","┗━━━━", "┗━━━┛","    ╹","┗━━━┛","┗━━━┛","    ╹","┗━━━┛","┗━━━┛","     "],
];
// Main
fn main() {
    // Setup
    let every_second = ScheduledTask::new(
        ScheduleAt::Interval(Duration::seconds(1)),
        task!(async { clock()})
    ).unwrap();
    let schedule = Schedule::builder()
        .tasks([
            every_second
        ])
        .wake_interval(Duration::seconds(1)).unwrap()
        .build();
    clear();
    // Run
    schedule.run();
    loop {}
}
// fn clear
fn clear() {
    Command::new("clear").status().unwrap();
}
// fn bulb
fn bulb (dig: &str, index: u8) -> String {
    match index {
        0 => String::from("  ----^----  "),
        1 => format!(" |  {}  | ", dig.truecolor(249,212,102)),
        2 => format!(" |  {}  | ", dig.truecolor(249,212,102)),
        3 => format!(" |  {}  | ", dig.truecolor(249,212,102)),
        4 => format!(" |  {}  | ", dig.truecolor(249,212,102)),
        5 => format!(" |  {}  | ", dig.truecolor(249,212,102)),
        6 => format!(" |  {}  | ", dig.truecolor(249,212,102)),
        7 => format!("  \\ {} /  ", dig.truecolor(249,212,102)),
        8 => String::from("  |||||||||  "),
        _ => {
            println!("Error aborting");
            exit(1);
        }
    }
}
// fn clock
fn clock () {
    clear();
    let now = Local::now();
    let time = now.format("%H:%M:%S");
    // print bulb
    for a in 0..9 {
        for _b in 0..8 {
            print!("{}", bulb("┏━━━┓", u8::from(a)))
        }
        println!();
    }
    // Box
    let date = now.format("%A :: %d %b %Y");
    let _mid = format!("{}", date.to_string().pad_to_width_with_alignment(68, Alignment::Middle).truecolor(249,212,102));
    let mid = format!("┃{}┃", _mid);
    println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("{}", mid);
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");
}
