
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone)]
pub enum EndReason {
    EndCode,
    CodeUnknown,
    EndProgram
}

#[derive(Eq, PartialEq, Copy, Debug, Clone)]
pub enum YieldState {
    Startup,
    Halted,
    InputWaiting
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Addressing {
    Position,
    Immediate,
    Relative
}

#[derive(Debug, Clone)]
pub struct OpCode {
    pub code: u8,
    pub addr_mode: Vec<Addressing>,
}

#[derive(Debug, Clone)]
pub struct ProgramResult {
    pub memory: Vec<i64>,
    pub outputs: Vec<i64>,
    pub code: EndReason
}

#[derive(Debug, Clone)]
pub struct Machine {
    pub counter: usize,
    pub state: YieldState,
    pub inputs: VecDeque<i64>,
    pub memory: Vec<i64>,
    pub outputs: Vec<i64>,
    pub code: Option<EndReason>,
    pub rel_base: i64,
}

impl Machine{
    pub fn get_result(self: &Self) -> Option<ProgramResult> {
        if self.state == YieldState::Halted && self.code.is_some() {
            return Option::from(ProgramResult {
                memory: self.memory.clone(),
                outputs: self.outputs.clone(),
                code: self.code.clone().unwrap()
            });
        }
        return Option::None;
    }

    #[allow(dead_code)]
    pub fn can_continue(self: &Self) -> bool {
        return self.state != YieldState::Halted;
    }
}

impl Addressing{
    pub fn from(addrmode: u8) -> Self {
        match addrmode {
            0 => return Addressing::Position,
            1 => return Addressing::Immediate,
            2 => return Addressing::Relative,

            _ => return Addressing::Position
        }
    }
}

impl OpCode{
    pub fn from(code: i64) -> Self {
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
        3 | 4 | 9 => 1,
        5 | 6  => 2,
        _ => 0
    }
}

pub fn split_string(string: String) -> Vec<i64>{
    let mut parts = Vec::new();
    for part in string.split(","){
        parts.push(part.parse().unwrap())
    }
    return parts;
}

pub fn ensure_memsize(state: &mut Machine, size: usize){
    if state.memory.len() >= size {
        return
    }

    let empty_space = size - state.memory.len();
    let mut empty_vec = Vec::with_capacity(empty_space);
    for _i in 0..empty_space {
        empty_vec.push(0);
    }
    state.memory.append(&mut empty_vec);
}

pub fn get_value(state: &Machine, addr: i64, mode: Addressing) -> i64{
    if mode == Addressing::Immediate {
        return addr;
    }

    let abs_addr: usize =  match mode {
        Addressing::Immediate => 0 ,
        Addressing::Position => addr as usize,
        Addressing::Relative => (addr + state.rel_base) as usize
    };

    if abs_addr >= state.memory.len() {
        return 0;
    }
    return state.memory[abs_addr];
}

pub fn set_value(state: &mut Machine, pos: u8, addr: i64, mode: Addressing, val: i64){
    let abs_addr: usize =  match mode {
        Addressing::Immediate => state.counter + pos as usize,
        Addressing::Position => addr as usize,
        Addressing::Relative => (addr + state.rel_base) as usize
    };

    ensure_memsize(state, abs_addr + 1);

    state.memory[abs_addr] = val;
}


pub fn run_program(prog_in: &Vec<i64>, input_in: &Vec<i64>) -> Machine {
    return continue_program(Machine {
        state: YieldState::Startup,
        counter: 0,
        memory: prog_in.clone(),
        inputs: input_in.clone().into_iter().collect(),
        outputs: Vec::new(),
        code: Option::None,
        rel_base: 0
    })
}

pub fn continue_program(mut state: Machine) -> Machine{

    if state.state == YieldState::Halted{
        return state;
    }

    let mut running = true;

    while running && state.counter < state.memory.len(){
        let code = OpCode::from(state.memory[state.counter]);

        match code.code {
            1 => {
                let a = state.memory[state.counter + 1];
                let b = state.memory[state.counter + 2];
                let c = state.memory[state.counter + 3];
                let val = get_value(&state, a, code.addr_mode[0]) + get_value(&state, b, code.addr_mode[1]);
                set_value(&mut state, 3, c, code.addr_mode[2], val);
                state.counter += 4
            },
            2 => {
                let a = state.memory[state.counter + 1];
                let b = state.memory[state.counter + 2];
                let c = state.memory[state.counter + 3];
                let val = get_value(&state, a, code.addr_mode[0]) * get_value(&state, b, code.addr_mode[1]);
                set_value(&mut state, 3, c, code.addr_mode[2], val);
                state.counter += 4
            },
            3 => {
                let a = state.memory[state.counter + 1];
                let in_opt = state.inputs.pop_front();
                if in_opt.is_some() {
                    set_value(&mut state, 1, a, code.addr_mode[0], in_opt.unwrap());
                }
                else {
                    state.state = YieldState::InputWaiting;
                    return state;
                }
                state.counter += 2;
            },
            4 => {
                let a = state.memory[state.counter + 1];
                state.outputs.push(get_value(&state, a, code.addr_mode[0]));
                state.counter += 2;
            },
            5 => {
                let a = state.memory[state.counter + 1];
                let b = state.memory[state.counter + 2];
                if get_value(&state, a, code.addr_mode[0]) != 0{
                    state.counter = get_value(&state, b, code.addr_mode[1]) as usize;
                }
                else {
                    state.counter += 3;
                }
            },
            6 => {
                let a = state.memory[state.counter + 1];
                let b = state.memory[state.counter + 2];
                if get_value(&state, a, code.addr_mode[0]) == 0{
                    state.counter = get_value(&state, b, code.addr_mode[1]) as usize;
                }
                else {
                    state.counter += 3;
                }
            },
            7 => {
                let a = state.memory[state.counter + 1];
                let b = state.memory[state.counter + 2];
                let c = state.memory[state.counter + 3];
                if get_value(&state, a, code.addr_mode[0]) < get_value(&state, b, code.addr_mode[1]){
                    set_value(&mut state, 3, c, code.addr_mode[2], 1);
                }
                else {
                    set_value(&mut state, 3, c, code.addr_mode[2], 0);
                }
                state.counter += 4;
            },
            8 => {
                let a = state.memory[state.counter + 1];
                let b = state.memory[state.counter + 2];
                let c = state.memory[state.counter + 3];
                if get_value(&state, a, code.addr_mode[0]) == get_value(&state, b, code.addr_mode[1]){
                    set_value(&mut state, 3, c, code.addr_mode[2], 1);
                }
                else {
                    set_value(&mut state, 3, c, code.addr_mode[2], 0);
                }
                state.counter += 4;
            },
            9 => {
                let a = state.memory[state.counter + 1];
                state.rel_base += get_value(&state, a, code.addr_mode[0]);
                state.counter += 2;
            },
            99 => {
                state.code = Option::from(EndReason::EndCode);
                running = false;
            },
            _ => {
                state.code = Option::from(EndReason::CodeUnknown);
                running = false;
            }
        }
    }

    if state.code.is_none() {
        state.code = Option::from(EndReason::EndProgram);
    }

    state.state = YieldState::Halted;
    return state;
}