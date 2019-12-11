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
enum Paint {
    BLACK,
    WHITE,
    NONE
}

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl Position2D {
    fn from(x: i16, y: i16) -> Self {
        return Position2D {
            x: x,
            y: y
        }
    }
}

impl Paint {
    fn from_intcode(code: i64) -> Self{
        return match code {
            0 => Self::BLACK,
            1 => Self::WHITE,
            _ => Self::NONE
        }
    }

    fn to_intcode(self: &Self) -> Option<i64>{
        return match self {
            Self::BLACK => Option::from(0),
            Self::WHITE => Option::from(1),
            _ => Option::None
        }
    }
}

impl Direction {
    fn turn_left(self: &Self) -> Self{
        return match self {
            Self::UP => Self::LEFT,
            Self::LEFT => Self::DOWN,
            Self::DOWN => Self::RIGHT,
            Self::RIGHT => Self::UP
        }
    }

    fn turn_right(self: &Self) -> Self{
        return match self {
            Self::UP => Self::RIGHT,
            Self::RIGHT => Self::DOWN,
            Self::DOWN => Self::LEFT,
            Self::LEFT => Self::UP
        }
    }

    fn move_pos(self: &Self, pos: &mut Position2D){
        match self {
            Self::UP => pos.y += 1,
            Self::RIGHT => pos.x += 1,
            Self::DOWN => pos.y -= 1,
            Self::LEFT => pos.x -= 1
        }
    }
}

fn print_img(img: &Vec<Vec<Paint>>){
    let mut img_string: String = String::new();
    for y in 0..img.len() {
        for x in 0..img[y].len() {
            if img[y][x] == Paint::WHITE{
                img_string += "█";
            }
            else {
                img_string += " ";
            }
        }
        img_string += "\n";
    }
    println!("{}", img_string)
}

fn map_to_img(img_map: &HashMap<Position2D, Paint>, background: Paint) -> Vec<Vec<Paint>>{
    let offset = calc_img_offset(&img_map);
    let size = calc_img_size(&img_map, &offset);
    let mut img: Vec<Vec<Paint>> = make_blank_img(&size, background);
    for (pos, paint) in img_map {
        img[(size.y - (pos.y + offset.y) - 1) as usize][(pos.x + offset.x) as usize] = *paint;
    }
    return img;
}

fn calc_img_offset(img_map: &HashMap<Position2D, Paint>) -> Position2D{
    let mut x = 0;
    let mut y = 0;
    for (pos, _paint) in img_map {
        if x > pos.x {
            x = pos.x;
        }

        if y > pos.y {
            y = pos.y;
        }
    }
    return Position2D::from(-x, -y);
}


fn calc_img_size(img_map: &HashMap<Position2D, Paint>, offset: &Position2D) -> Position2D{
    let mut x = 0;
    let mut y = 0;
    for (pos, _paint) in img_map {
        let tx = pos.x + offset.x;
        let ty = pos.y + offset.y;
        if x < tx {
            x = tx;
        }

        if y < ty {
            y = ty
        }
    }
    return Position2D::from(x+1, y+1);
}

fn make_blank_img(size: &Position2D, background: Paint) -> Vec<Vec<Paint>>{
    let mut img: Vec<Vec<Paint>> = Vec::new();
    for _y in 0..size.y {
        let mut col = Vec::new();
        for _x in 0..size.x {
            col.push(background);
        }   
        img.push(col)
    }
    return img;
}

fn run_painter(mem: &Vec<i64>, background: Paint) -> HashMap<Position2D, Paint>{
    let mut img: HashMap<Position2D, Paint> = HashMap::new();
    let mut pos = Position2D::from(0, 0);
    let mut dir = Direction::UP;
    let mut machine = Machine::make(mem, &vec![]);

    let default_color = background.to_intcode().unwrap_or(0);

    while machine.can_continue() {
        let paint = img.get(&pos);
        let mut paint_code = Option::None;
        if paint.is_some() {
            paint_code = paint.unwrap().to_intcode();
        }

        machine.push_input(&vec![paint_code.unwrap_or(default_color)]);
        machine.continue_program();

        let outlen = machine.outputs.len();
        img.insert(pos, Paint::from_intcode(machine.outputs[outlen - 2]));
        match machine.outputs[outlen - 1] {
            0 => dir = dir.turn_left(),
            1 => dir = dir.turn_right(),
            _ => {}
        }
        dir.move_pos(&mut pos);
    }
    return img;
}

fn solve_puzzle(mem: &Vec<i64>) {
    let img = run_painter(&mem, Paint::BLACK);
    println!("1.) {}", img.len());

    let img2 = run_painter(&mem, Paint::WHITE);
    let img_arr = map_to_img(&img2, Paint::WHITE);
    println!("2.) <image below; 8 capital letters>");
    print_img(&img_arr);
}

fn main(){
    title::print_title(11, "Space Police");
    let mem = split_string(fs::read_to_string("day_11/program.txt").expect("File error!"));
    solve_puzzle(&mem);
}