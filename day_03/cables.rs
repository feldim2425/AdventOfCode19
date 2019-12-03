extern crate regex;


use std::vec::Vec;
use std::string::String;
use self::regex::Regex;

lazy_static! {
    static ref CABELE_REGEX : Regex = Regex::new(r"(?m)([LRUD])(\d*)").unwrap();
}

#[derive(Eq, PartialEq, Clone, Debug, Copy)]
pub enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT
}

#[derive(Eq, PartialEq, Clone, Debug, Copy)]
pub enum Orientaition {
    Horizontal,
    Vertical
}

#[derive(Clone, Debug)]
pub struct CableStep {
    pub direction: Direction,
    pub steps: i32
}

pub fn parse_step(step: String) -> CableStep {
    let caps = CABELE_REGEX.captures(&step).unwrap();
    let steps = caps.get(2).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
    let mut direction = Direction::DOWN;
    match caps.get(1).unwrap().as_str().as_ref() {
        "L" => direction = Direction::LEFT,
        "R" => direction = Direction::RIGHT,
        "U" => direction = Direction::UP,
        "D" => direction = Direction::DOWN,
        _ => {}
    }

    return CableStep {
        steps: steps,
        direction: direction
    }
}

pub fn parse_steps(steps: Vec<String>) -> Vec<CableStep> {
    let mut step_list = Vec::new();
    for step in steps {
        step_list.push(parse_step(step));
    }
    return step_list;
}

pub fn get_orientation(dir: Direction) -> Orientaition{
    match dir {
        Direction::LEFT => return Orientaition::Horizontal,
        Direction::RIGHT => return Orientaition::Horizontal,
        Direction::UP => return Orientaition::Vertical,
        Direction::DOWN => return Orientaition::Vertical,
    }
}