mod cables;

#[macro_use]
extern crate lazy_static;

use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;
use std::fs::File;
use std::string::String;
use std::vec::Vec;

#[derive(Eq, PartialEq, Clone, Debug, Copy)]
pub struct Line2D {
    start: Point2D,
    dir : cables::Direction,
    len: i32
}

#[derive(Eq, PartialEq, Clone, Debug, Copy)]
pub struct Point2D {
    x: i32,
    y: i32,
}

fn read_puzzle_input() -> Result<Vec<Vec<String>>> {
    let file = File::open("day_03/input.txt")?;
    let buffer = BufReader::new(file);
    let mut cables : Vec<Vec<String>> = Vec::new();

    for line in buffer.lines() {
        let mut steps : Vec<String> = Vec::new();
        for step in line.unwrap().split(",") {
            steps.push(step.to_string());
        }
        cables.push(steps);
    }

    Ok(cables)
}

fn interpolate_cable(cable : & [cables::CableStep]) -> Vec<Line2D>{
    let mut current = Point2D { x:0, y:0 };
    let mut lines : Vec<Line2D> = Vec::new();
    for step in cable {
        lines.push(Line2D {
            start: current.clone(),
            dir: step.direction,
            len: step.steps
        });
        match step.direction{
            cables::Direction::DOWN => current.y -= step.steps,
            cables::Direction::UP => current.y += step.steps,
            cables::Direction::LEFT => current.x -= step.steps,
            cables::Direction::RIGHT => current.x += step.steps,
        }
    }
    return lines;
}

fn check_possible_intersection(a: Line2D, b: Line2D) -> bool {
    match a.dir {
        cables::Direction::RIGHT => {
            if a.start.x <= b.start.x && a.start.x + a.len > b.start.x{
                return true;
            }
        }
        cables::Direction::LEFT => {
            if a.start.x >= b.start.x && a.start.x - a.len < b.start.x{
                return true;
            }
        }
        cables::Direction::UP => {
            if a.start.y <= b.start.y && a.start.y + a.len > b.start.y{
                return true;
            }
        }
        cables::Direction::DOWN => {
            if a.start.y >= b.start.y && a.start.y - a.len < b.start.y{
                return true;
            }
        }
    }
    return false;
}

fn cross_lines(a: Line2D, b: Line2D) -> Option<Point2D> {
    if cables::get_orientation(a.dir) == cables::get_orientation(b.dir) {
        return Option::None;
    }

    if !check_possible_intersection(a, b) || !check_possible_intersection(b, a){
        return Option::None;
    }

    if cables::get_orientation(a.dir) == cables::Orientaition::Horizontal {
        return Option::from( Point2D {
            x: b.start.x,
            y: a.start.y
        })
    }
    else {
        return Option::from( Point2D {
            x: a.start.x,
            y: b.start.y
        })
    }
}

fn calc_steps(path: &Vec<Line2D>, cur_line: Line2D, pt: Point2D) -> i32 {
    let mut steps : i32 = 0;
    for line in path {
        steps += line.len;
    }

    if cables::get_orientation(cur_line.dir) == cables::Orientaition::Horizontal {
        steps += i32::abs(cur_line.start.x - pt.x)
    }
    else {
        steps += i32::abs(cur_line.start.y - pt.y)
    }

    return steps;
}


fn solve_puzzle(cables : Vec<Vec<String>>) {
    let mut cables_parsed : Vec<Vec<cables::CableStep>> = Vec::new();
    for cable in cables {
        cables_parsed.push(cables::parse_steps(cable));
    }

    let mut smallest_dist = std::i32::MAX;
    let mut smallest_path = std::i32::MAX;
    let lines1 = interpolate_cable(&cables_parsed[0].to_vec());
    let lines2 = interpolate_cable(&cables_parsed[1].to_vec());

    let mut path1 : Vec<Line2D> = Vec::new();
    for line_a in &lines1{
        let mut path2 :  Vec<Line2D> = Vec::new();
        for line_b in &lines2{
            let opt_pt : Option<Point2D> = cross_lines(line_a.clone(), line_b.clone());
            if opt_pt.is_some() {
                let p2d = opt_pt.unwrap();

                let dist : i32 = i32::abs(p2d.x) + i32::abs(p2d.y);
                if dist != 0 {
                    if dist < smallest_dist{
                        smallest_dist = dist;
                    }

                    let steps = calc_steps(&path1, line_a.clone(), p2d) + calc_steps(&path2, line_b.clone(), p2d);
                    if steps < smallest_path{
                        smallest_path = steps;
                    }
                }
            }

            path2.push(line_b.clone());
        }
        path1.push(line_a.clone());
    }

    println!("1.) {}", smallest_dist);
    println!("2.) {}", smallest_path);
}

fn main(){
    match read_puzzle_input(){
        Ok(res) => solve_puzzle(res),
        Err(res) => println!("Error while reading puzzle input! {}", res),
    }
}