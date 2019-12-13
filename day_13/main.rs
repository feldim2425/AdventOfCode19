#[path = "../common/title.rs"]
mod title;
mod intcomputer;

use std::fs;
use intcomputer::*;
use std::collections::HashMap;

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
struct Position2D {
    x: i16,
    y: i16,
}

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
enum Tile {
    EMPTY,
    WALL,
    BLOCK,
    HPADDLE,
    BALL
}

impl Position2D {
    fn from(x: i16, y: i16) -> Self {
        return Position2D {
            x: x,
            y: y
        }
    }
}

impl Tile {
    fn from_intcode(code: i64) -> Self{
        return match code {
            0 => Self::EMPTY,
            1 => Self::WALL,
            2 => Self::BLOCK,
            3 => Self::HPADDLE,
            4 => Self::BALL,
            _ => Self::EMPTY
        }
    }

    #[allow(dead_code)]
    fn to_intcode(self: &Self) -> Option<i64>{
        return match self {
            Self::EMPTY => Option::from(0),
            Self::WALL => Option::from(1),
            Self::BLOCK => Option::from(2),
            Self::HPADDLE => Option::from(3),
            Self::BALL => Option::from(4),
        }
    }
}

fn run_game_1(mem: &Vec<i64>) -> HashMap<Position2D, Tile>{
    let mut field: HashMap<Position2D, Tile> = HashMap::new();

    let mut machine = Machine::make(mem, &vec![]);
    machine.continue_program();

    let mut count = 0;
    while count < machine.outputs.len() {
        field.insert(Position2D::from(machine.outputs[count] as i16, machine.outputs[count + 1] as i16), Tile::from_intcode(machine.outputs[count + 2]));
        count += 3;
    }
    return field;
}

fn run_game_2(mem: &Vec<i64>) -> u16{
    let mut field: HashMap<Position2D, Tile> = HashMap::new();

    let mut machine = Machine::make(mem, &vec![]);
    let mut score = 0;
    machine.memory[0] = 2;
    
    while machine.can_continue() {
        machine.continue_program();

        let mut count = 0;
        while count < machine.outputs.len() {
            let pos = Position2D::from(machine.outputs[count] as i16, machine.outputs[count + 1] as i16);
            if pos.x == -1 && pos.y == 0 {
                score = machine.outputs[count + 2] as u16;
            }
            else {
                field.insert(pos, Tile::from_intcode(machine.outputs[count + 2]));
            }
            count += 3;
        }
        machine.outputs.clear();

        let ball: Option<(&Position2D, &Tile)> = field.iter().find(|&(_k, v)| *v == Tile::BALL);
        let paddle: Option<(&Position2D, &Tile)> = field.iter().find(|&(_k, v)| *v == Tile::HPADDLE);
        let mut in_key = 0;
        if ball.is_some() && paddle.is_some() {
            if ball.unwrap().0.x < paddle.unwrap().0.x {
                in_key = -1;
            }
            else if ball.unwrap().0.x > paddle.unwrap().0.x {
                in_key = 1;
            }
        }
        machine.push_input(&vec![in_key]);
    }
    return score;
}

fn solve_puzzle(mem: &Vec<i64>) {
    let field = run_game_1(&mem);
    let score = run_game_2(&mem);
    println!("1.) {}", field.iter().filter(|&(_k, v)| *v == Tile::BLOCK).count());
    println!("2.) {}", score);
}

fn main(){
    
    title::print_title(13, "Care Package");
    let mem = split_string(fs::read_to_string("day_13/program.txt").expect("File error!"));
    solve_puzzle(&mem);
}