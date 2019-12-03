use std::vec::Vec;
use std::string::String;

#[derive(Debug)]
enum EndReason {
    EndCode,
    CodeUnknown,
    EndProgram
}

#[derive(Debug)]
struct ProgramResult {
    memory: Vec<i32>,
    code: EndReason,
}

const PROGRAM : &'static str = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,13,1,19,1,5,19,23,2,10,23,27,1,27,5,31,2,9,31,35,1,35,5,39,2,6,39,43,1,43,5,47,2,47,10,51,2,51,6,55,1,5,55,59,2,10,59,63,1,63,6,67,2,67,6,71,1,71,5,75,1,13,75,79,1,6,79,83,2,83,13,87,1,87,6,91,1,10,91,95,1,95,9,99,2,99,13,103,1,103,6,107,2,107,6,111,1,111,2,115,1,115,13,0,99,2,0,14,0";


fn split_string(string: String) -> Vec<i32>{
    let mut parts = Vec::new();
    for part in string.split(","){
        parts.push(part.parse().unwrap())
    }
    return parts;
}

fn run_program(mut prog: Vec<i32>) -> ProgramResult{
    let mut counter = 0;
    let mut running = true;
    let mut end_reason = EndReason::EndProgram;

    while running && counter < prog.len(){
        match prog[counter] {
            1 => {
                let a = prog[counter + 1];
                let b = prog[counter + 2];
                let c = prog[counter + 3];
                prog[c as usize] = prog[b as usize] + prog[a as usize];
                counter += 4
            },
            2 => {
                let a = prog[counter + 1];
                let b = prog[counter + 2];
                let c = prog[counter + 3];
                prog[c as usize] = prog[b as usize] * prog[a as usize];
                counter += 4
            },
            99 => {
                end_reason = EndReason::EndCode;
                running = false;
            },
            _ => {
                end_reason = EndReason::CodeUnknown;
                running = false;
            }
        }
    }

    return ProgramResult {
        memory: prog,
        code: end_reason
    }
}

fn main(){
    let mem = split_string(String::from(PROGRAM));
    let mut mem_1 = mem.clone();
    mem_1[1] = 12;
    mem_1[2] = 2;
    let result_1 = run_program(mem_1);
    println!("1.) {}", result_1.memory[0]);

    for noun in 0..99{
        for verb in 0..99{
            let mut mem_2 = mem.clone();
            mem_2[1] = noun;
            mem_2[2] = verb;
            let result_2 = run_program(mem_2);
            if result_2.memory[0] == 19690720 {
                println!("2.) {}", noun * 100 + verb);
                break;
            }
        }
    }
    
}