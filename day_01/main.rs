use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;
use std::fs::File;
use std::vec::Vec;

fn read_puzzle_input() -> Result<Vec<i32>> {
    let file = File::open("day_01/input.txt")?;
    let buffer = BufReader::new(file);
    let mut masses = Vec::new();

    for line in buffer.lines() {
        masses.push(line.unwrap().parse::<i32>().unwrap());
    }

    Ok(masses)
}

fn fuel_from_mass(mass: i32) -> i32 {
    return mass / 3 - 2;
}

fn solve_puzzle(masses: Vec<i32>) {

    let mut fuel_modules = 0;
    let mut fuel_total = 0;
    for mass in masses {
        let mut fuel = fuel_from_mass(mass);
        fuel_modules += fuel;

        let mut added = fuel;
        loop {
            added = fuel_from_mass(added);
            if added <= 0{
                break;
            }
            fuel += added;
        }
        fuel_total += fuel;
    }

    println!("1.) Fuel required for all the modules: {}",fuel_modules);
    println!("2.) Fuel required with the added fuel is: {}",fuel_total);
}

fn main(){
    match read_puzzle_input(){
        Ok(res) => solve_puzzle(res),
        Err(res) => println!("Error while reading puzzle input! {}", res),
    }
}