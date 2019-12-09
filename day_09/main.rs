mod intcomputer;

use std::fs;
use intcomputer::*;

fn solve_puzzle(mem: &Vec<i64>) {
    let result = run_program(mem, &vec![1]);
    println!("1.) {}", result.get_result().unwrap().outputs[0]);

    let result_2 = run_program(mem, &vec![2]);
    println!("2.) {}", result_2.get_result().unwrap().outputs[0]);
}

fn main(){
    let mem = split_string(fs::read_to_string("day_09/program.txt").expect("File error!"));
    solve_puzzle(&mem);
}