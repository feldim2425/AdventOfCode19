mod intcomputer;

use std::fs;
use intcomputer::*;

fn next_setting(set: &mut Vec<u8>, offset: u8) -> bool {
    let mut i :usize = 0;
    loop {
        set[i] += 1;
        if set[i] > 4 + offset {
            set[i] = offset;
            i += 1;
            if i >= set.len() {
                return false;
            }
        }
        else {
            break;
        }
    }
    return true;
}

fn check_valid(set: &Vec<u8>) -> bool {
    for i in 0..set.len() {
        for y in 0..set.len() {
            if i != y && set[i] == set[y] {
                return false;
            }
        }
    }
    return true;
}

fn run_amp_chain_1(mem: &Vec<i32>, settings: &Vec<u8>) -> i32 {
    let mut signal = 0;
    for set_val in settings {
        let result = run_program(mem, &vec![*set_val as i32, signal]).get_result().unwrap();
        signal = result.outputs[0];
    }
    
    return signal;
}

fn run_amp_chain_2(mem: &Vec<i32>, settings: &Vec<u8>) -> i32 {
    let mut signal = 0;
    let mut states: Vec<ProgramState> = Vec::new();
    let mut ended = false;

    for set_val in settings {
        let result = run_program(mem, &vec![*set_val as i32 + 5, signal]);
        signal = result.outputs[0];
        if !result.can_continue() {
            ended = true;
            break;
        }
        states.push(result);
    }

    while !ended {
        for i in 0..states.len() {
            states[i].inputs.push_back(signal);
            let result = continue_program(states[i].clone());
            signal = result.outputs[result.outputs.len()-1];
            if !result.can_continue() {
                ended = true;
            }
            states[i] = result;
        }
    }
    
    return signal;
}

fn main(){
    let mem = split_string(fs::read_to_string("day_07/program.txt").expect("File error!"));

    let mut settings = vec![0,0,0,0,0];
    let mut max_val = 0;
    let mut max_val2 = 0;
    'outer: loop {
        while !check_valid(&settings){
            if !next_setting(&mut settings, 0) {
                break 'outer;
            }
        }

        let signal = run_amp_chain_1(&mem, &settings);
        if signal > max_val {
            max_val = signal;
        }

        let signal2 = run_amp_chain_2(&mem, &settings);
        if signal2 > max_val2 {
            max_val2 = signal2;
        }

        if !next_setting(&mut settings, 0) {
            break 'outer;
        }
    }

    println!("1.) {}", max_val);
    println!("2.) {}", max_val2);
}