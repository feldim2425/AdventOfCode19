extern crate colored;

use self::colored::*;

pub fn print_title(day: u8, title: &str){
    for _i in 0..3 {
        print!("{}", "~".white());
        print!("{}", "=".red());
    }

    print!("{}", " Advent of Code ".green());
    print!("{}", "2019".bright_green().bold());
    print!("{}", " : ".red());
    print!("{} {}", "Day".cyan(), format!("{}", day).bright_cyan().bold());
    print!("{}", " : ".red());
    print!("{} ", format!("{}", title).purple());

    for _i in 0..3 {
        print!("{}", "=".red());
        print!("{}", "~".white());
    }
    println!("");
}