
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
    pub fn make(prog_in: &Vec<i64>, input_in: &Vec<i64>) -> Self { 
        return Self {
            state: YieldState::Startup,
            counter: 0,
            memory: prog_in.clone(),
            inputs: input_in.clone().into_iter().collect(),
            outputs: Vec::new(),
            code: Option::None,
            rel_base: 0
        };;
    }

    #[allow(dead_code)]
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

    pub fn push_input(self: &mut Self, inputs: &Vec<i64>){
        self.inputs.append(&mut inputs.clone().into_iter().collect());
    }

    /**
     * Get value based on address/argument specified and the mode
     */
    pub fn get_value(self: &Self, addr: i64, mode: Addressing) -> i64{
        if mode == Addressing::Immediate {
            return addr;
        }
    
        let abs_addr: usize =  match mode {
            Addressing::Immediate => 0 ,
            Addressing::Position => addr as usize,
            Addressing::Relative => (addr + self.rel_base) as usize
        };
    
        if abs_addr >= self.memory.len() {
            return 0;
        }
        return self.memory[abs_addr];
    }
    
    /**
     * Set value based on counter position, argument position, address specified and the mode
     */
    pub fn set_value(self: &mut Self, pos: u8, addr: i64, mode: Addressing, val: i64){
        let abs_addr: usize =  match mode {
            Addressing::Immediate => self.counter + pos as usize,
            Addressing::Position => addr as usize,
            Addressing::Relative => (addr + self.rel_base) as usize
        };
    
        ensure_memsize(self, abs_addr + 1);
    
        self.memory[abs_addr] = val;
    }

    /**
     * Run the program in the machine until the next yield occurs or the end is reached
     */
    pub fn continue_program(self: &mut Self){
        if self.state == YieldState::Halted{
            return;
        }
    
        let mut running = true;
    
        while running && self.counter < self.memory.len(){
            let code = OpCode::from(self.memory[self.counter]);
    
            match code.code {
                1 => {
                    let a = self.memory[self.counter + 1];
                    let b = self.memory[self.counter + 2];
                    let c = self.memory[self.counter + 3];
                    let val = self.get_value(a, code.addr_mode[0]) + self.get_value(b, code.addr_mode[1]);
                    self.set_value(3, c, code.addr_mode[2], val);
                    self.counter += 4
                },
                2 => {
                    let a = self.memory[self.counter + 1];
                    let b = self.memory[self.counter + 2];
                    let c = self.memory[self.counter + 3];
                    let val = self.get_value(a, code.addr_mode[0]) * self.get_value(b, code.addr_mode[1]);
                    self.set_value(3, c, code.addr_mode[2], val);
                    self.counter += 4
                },
                3 => {
                    let a = self.memory[self.counter + 1];
                    let in_opt = self.inputs.pop_front();
                    if in_opt.is_some() {
                        self.set_value(1, a, code.addr_mode[0], in_opt.unwrap());
                    }
                    else {
                        self.state = YieldState::InputWaiting;
                        return;
                    }
                    self.counter += 2;
                },
                4 => {
                    let a = self.memory[self.counter + 1];
                    self.outputs.push(self.get_value(a, code.addr_mode[0]));
                    self.counter += 2;
                },
                5 => {
                    let a = self.memory[self.counter + 1];
                    let b = self.memory[self.counter + 2];
                    if self.get_value(a, code.addr_mode[0]) != 0{
                        self.counter = self.get_value(b, code.addr_mode[1]) as usize;
                    }
                    else {
                        self.counter += 3;
                    }
                },
                6 => {
                    let a = self.memory[self.counter + 1];
                    let b = self.memory[self.counter + 2];
                    if self.get_value(a, code.addr_mode[0]) == 0{
                        self.counter = self.get_value(b, code.addr_mode[1]) as usize;
                    }
                    else {
                        self.counter += 3;
                    }
                },
                7 => {
                    let a = self.memory[self.counter + 1];
                    let b = self.memory[self.counter + 2];
                    let c = self.memory[self.counter + 3];
                    if self.get_value(a, code.addr_mode[0]) < self.get_value(b, code.addr_mode[1]){
                        self.set_value(3, c, code.addr_mode[2], 1);
                    }
                    else {
                        self.set_value(3, c, code.addr_mode[2], 0);
                    }
                    self.counter += 4;
                },
                8 => {
                    let a = self.memory[self.counter + 1];
                    let b = self.memory[self.counter + 2];
                    let c = self.memory[self.counter + 3];
                    if self.get_value(a, code.addr_mode[0]) == self.get_value(b, code.addr_mode[1]){
                        self.set_value(3, c, code.addr_mode[2], 1);
                    }
                    else {
                        self.set_value(3, c, code.addr_mode[2], 0);
                    }
                    self.counter += 4;
                },
                9 => {
                    let a = self.memory[self.counter + 1];
                    self.rel_base += self.get_value(a, code.addr_mode[0]);
                    self.counter += 2;
                },
                99 => {
                    self.code = Option::from(EndReason::EndCode);
                    running = false;
                },
                _ => {
                    self.code = Option::from(EndReason::CodeUnknown);
                    running = false;
                }
            }
        }
    
        if self.code.is_none() {
            self.code = Option::from(EndReason::EndProgram);
        }
    
        self.state = YieldState::Halted;
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

fn get_opcode_params(opcode: u8) -> u8{
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

fn ensure_memsize(state: &mut Machine, size: usize){
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

#[allow(dead_code)]
pub fn run_program(prog_in: &Vec<i64>, input_in: &Vec<i64>) -> Machine {
    let mut machine = Machine::make(&prog_in, &input_in);
    machine.continue_program();
    return machine;
}