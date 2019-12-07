
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone)]
pub enum EndReason {
    EndCode,
    CodeUnknown,
    EndProgram
}

#[derive(Eq, PartialEq, Copy, Debug, Clone)]
pub enum YieldReason {
    Startup,
    Halted,
    InputWaiting
}

#[derive(Debug, Copy, Clone)]
pub enum Addressing {
    Position,
    Immediate
}

#[derive(Debug, Clone)]
pub struct OpCode {
    pub code: u8,
    pub addr_mode: Vec<Addressing>,
}

#[derive(Debug, Clone)]
pub struct ProgramResult {
    pub memory: Vec<i32>,
    pub outputs: Vec<i32>,
    pub code: EndReason
}

#[derive(Debug, Clone)]
pub struct ProgramState {
    pub counter: usize,
    pub state: YieldReason,
    pub inputs: VecDeque<i32>,
    pub memory: Vec<i32>,
    pub outputs: Vec<i32>,
    pub code: Option<EndReason>
}

impl ProgramState{
    pub fn get_result(self: &Self) -> Option<ProgramResult> {
        if self.state == YieldReason::Halted && self.code.is_some() {
            return Option::from(ProgramResult {
                memory: self.memory.clone(),
                outputs: self.outputs.clone(),
                code: self.code.clone().unwrap()
            });
        }
        return Option::None;
    }

    pub fn can_continue(self: &Self) -> bool {
        return self.state != YieldReason::Halted;
    }
}

impl Addressing{
    pub fn from(addrmode: u8) -> Self {
        match addrmode {
            0 => return Addressing::Position,
            1 => return Addressing::Immediate,

            _ => return Addressing::Position
        }
    }
}

impl OpCode{
    pub fn from(code: i32) -> Self {
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

pub fn get_opcode_params(opcode: u8) -> u8{
    return match opcode {
        1 | 2 | 7 | 8 => 3,
        3 | 4 => 1,
        5 | 6  => 2,
        _ => 0
    }
}

pub fn split_string(string: String) -> Vec<i32>{
    let mut parts = Vec::new();
    for part in string.split(","){
        parts.push(part.parse().unwrap())
    }
    return parts;
}

pub fn get_value(mem: &Vec<i32>, val: i32, mode: Addressing) -> i32{
    return match mode {
        Addressing::Immediate => val,
        Addressing::Position => mem[val as usize]
    }
}

pub fn run_program(prog_in: &Vec<i32>, input_in: &Vec<i32>) -> ProgramState {
    return continue_program(ProgramState {
        state: YieldReason::Startup,
        counter: 0,
        memory: prog_in.clone(),
        inputs: input_in.clone().into_iter().collect(),
        outputs: Vec::new(),
        code: Option::None
    })
}

pub fn continue_program(state: ProgramState) -> ProgramState{

    if state.state == YieldReason::Halted{
        return state;
    }

    let mut prog = state.memory;
    let mut input = state.inputs;
    let mut outputs = state.outputs;
    let mut counter = state.counter;
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
                let in_opt = input.pop_front();
                if in_opt.is_some() {
                    prog[a as usize] = in_opt.unwrap();
                }
                else {
                    return ProgramState {
                        counter: counter,
                        state: YieldReason::InputWaiting,
                        inputs: input,
                        memory: prog,
                        code: Option::None,
                        outputs: outputs
                    }
                }
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

    return ProgramState {
        counter: counter,
        state: YieldReason::Halted,
        inputs: input,
        memory: prog,
        code: Option::from(end_reason),
        outputs: outputs
    }
}