#[macro_use] extern crate text_io;

#[path = "../common/title.rs"]
mod title;

use std::vec::Vec;
use std::io::{self, Write};
use std::fs;
use std::string::String;

#[derive(Debug)]
enum EndReason {
    EndCode,
    CodeUnknown,
    EndProgram
}

#[derive(Debug, Copy, Clone)]
enum Addressing {
    Position,
    Immediate
}

#[derive(Debug)]
struct ProgramResult {
    memory: Vec<i32>,
    outputs: Vec<i32>,
    code: EndReason,
}

#[derive(Debug)]
struct OpCode {
    code: u8,
    addr_mode: Vec<Addressing>,
}

impl Addressing{
    fn from(addrmode: u8) -> Self {
        match addrmode {
            0 => return Addressing::Position,
            1 => return Addressing::Immediate,

            _ => return Addressing::Position
        }
    }
}

impl OpCode{
    fn from(code: i32) -> Self {
        let opcode = (code % 100) as u8;
        let mut modes = code / 100;
        let mut mode_arr: Vec<Addressing> = Vec::new();
        for _i in 0..get_opcode_params(opcode) {
            mode_arr.push(Addressing::from((modes % 10) as u8));
            modes = modes / 10;
        }

        return OpCode {
            code: opcode,
            addr_mode: mode_arr
        }
    }
}

fn get_opcode_params(opcode: u8) -> u8{
    return match opcode {
        1 | 2 | 7 | 8 => 3,
        3 | 4 => 1,
        5 | 6  => 2,
        _ => 0
    }
}

fn split_string(string: String) -> Vec<i32>{
    let mut parts = Vec::new();
    for part in string.split(","){
        parts.push(part.parse().unwrap())
    }
    return parts;
}

fn get_value(mem: &Vec<i32>, val: i32, mode: Addressing) -> i32{
    return match mode {
        Addressing::Immediate => val,
        Addressing::Position => mem[val as usize]
    }
}

fn run_program(prog_in: &Vec<i32>) -> ProgramResult{
    let mut prog = prog_in.clone();
    let mut outputs: Vec<i32> = Vec::new();
    let mut counter = 0;
    let mut running = true;
    let mut end_reason = EndReason::EndProgram;

    while running && counter < prog.len(){
        let code = OpCode::from(prog[counter]);

        match code.code {
            1 => {
                let a = prog[counter + 1];
                let b = prog[counter + 2];
                let c = prog[counter + 3];
                prog[c as usize] = get_value(&prog, a, code.addr_mode[0]) + get_value(&prog, b, code.addr_mode[1]);
                counter += 4
            },
            2 => {
                let a = prog[counter + 1];
                let b = prog[counter + 2];
                let c = prog[counter + 3];
                prog[c as usize] = get_value(&prog, a, code.addr_mode[0]) * get_value(&prog, b, code.addr_mode[1]);
                counter += 4
            },
            3 => {
                let a = prog[counter + 1];
                print!("Input required: ");
                io::stdout().flush().unwrap();
                prog[a as usize] = read!();
                counter += 2;
            },
            4 => {
                let a = prog[counter + 1];
                outputs.push(get_value(&prog, a, code.addr_mode[0]));
                counter += 2;
            },
            5 => {
                let a = prog[counter + 1];
                let b = prog[counter + 2];
                if get_value(&prog, a, code.addr_mode[0]) != 0{
                    counter = get_value(&prog, b, code.addr_mode[1]) as usize;
                }
                else {
                    counter += 3;
                }
            },
            6 => {
                let a = prog[counter + 1];
                let b = prog[counter + 2];
                if get_value(&prog, a, code.addr_mode[0]) == 0{
                    counter = get_value(&prog, b, code.addr_mode[1]) as usize;
                }
                else {
                    counter += 3;
                }
            },
            7 => {
                let a = prog[counter + 1];
                let b = prog[counter + 2];
                let c = prog[counter + 3];
                if get_value(&prog, a, code.addr_mode[0]) < get_value(&prog, b, code.addr_mode[1]){
                    prog[c as usize] = 1
                }
                else {
                    prog[c as usize] = 0
                }
                counter += 4;
            },
            8 => {
                let a = prog[counter + 1];
                let b = prog[counter + 2];
                let c = prog[counter + 3];
                if get_value(&prog, a, code.addr_mode[0]) == get_value(&prog, b, code.addr_mode[1]){
                    prog[c as usize] = 1
                }
                else {
                    prog[c as usize] = 0
                }
                counter += 4;
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
        code: end_reason,
        outputs: outputs
    }
}

fn main(){
    title::print_title(5, "Sunny with a Chance of Asteroids");
    let mem = split_string(fs::read_to_string("day_05/program.txt").expect("File error!"));
    println!("=> Part 1 ... [Enter \"1\" to continue]");
    let result_1 = run_program(&mem);
    println!("1.) {}", result_1.outputs.iter().find(|&&x| x != 0).unwrap());
    println!("=> Part 2 ... [Enter \"5\" to continue]");
    let result_2 = run_program(&mem);
    println!("2.) {}", result_2.outputs.iter().find(|&&x| x != 0).unwrap());
}