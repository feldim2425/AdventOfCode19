#[path = "../common/title.rs"]
mod title;

extern crate ordered_float;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use ordered_float::OrderedFloat;

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
struct Position2D {
    x: u16,
    y: u16,
}

#[derive(Debug,Copy,Clone)]
struct Astroid {
    distance: f32,
    pos: Position2D,
}

fn read_puzzle_input() -> std::io::Result<Vec<Position2D>> {
    let file = File::open("day_10/input.txt")?;
    let buffer = BufReader::new(file);
    let mut astroids : Vec<Position2D> = Vec::new();

    let mut y: u16 = 0;
    for line in buffer.lines() {
        let uline = line.unwrap();
        
        let mut x: u16 = 0;
        for ch in uline.chars() {
            if ch == '#' {
                astroids.push(Position2D {
                    x: x,
                    y: y
                })
            }
            x += 1;
        }

        y += 1;
    }

    Ok(astroids)
}

fn calc_angle(station: &Position2D, astroid: &Position2D) -> f32{
    return (station.y  as f32 - astroid.y  as f32).atan2(station.x  as f32 - astroid.x  as f32);
}

fn calc_distance(station: &Position2D, astroid: &Position2D) -> f32{
    let x = station.x  as f32 - astroid.x  as f32;
    let y = station.y  as f32 - astroid.y  as f32;
    return (x*x + y*y).sqrt();
}

fn get_inline(station: &Position2D, astroids: &Vec<Position2D>) -> Vec<Position2D> {
    let mut dist_map: HashMap<OrderedFloat<f32>, Astroid> = HashMap::new();
    for astroid in astroids {
        if astroid == station {
            continue;
        }

        let angle = calc_angle(&station, &astroid);
        let dist = calc_distance(station, &astroid);
        let astr = dist_map.get(&OrderedFloat::from(angle));

        if astr.is_none() || (astr.is_some() && astr.unwrap().distance > dist) {
            dist_map.insert(OrderedFloat::from(angle), Astroid {
                distance: dist,
                pos: astroid.clone()
            });
        }
    }

    let mut positions = Vec::new();
    for (_a, astroid) in dist_map.into_iter(){
        positions.push(astroid.pos);
    }

    return positions;
}

#[allow(dead_code)]
fn print_debug(base: &Position2D, eliminated: &Vec<Position2D>){
    println!("Base at {},{}",base.x, base.y);

    for i in 0..eliminated.len() {
        println!("{} -> {},{}",i, eliminated[i].x, eliminated[i].y);
    }
}

fn relative_rotation(mut angle: f32, rotation: f32) -> f32{
    angle -= std::f32::consts::PI / 2f32;
    angle -= rotation;
    while angle < 0f32 {
        angle += std::f32::consts::PI * 2f32;
    }
    return angle;
}

fn vaporization_procedure(station: &Position2D, astroids_in: &Vec<Position2D>) -> Vec<Position2D>{
    let mut astroids: HashSet<Position2D> = HashSet::new();
    for astroid in astroids_in {
        astroids.insert(astroid.clone());
    }
    
    let mut vaporized = Vec::new();
    let mut station_rot = 0.0f32;

    while !astroids.is_empty() {
        let mut inline = get_inline(&station, &astroids.clone().into_iter().collect());
        inline.sort_by(|a, b| relative_rotation(calc_angle(&station, a),station_rot).partial_cmp(&relative_rotation(calc_angle(&station, b), station_rot)).unwrap());
        
        if inline.len() == 0 {
            break;
        }

        for vap_astr in &inline {
            vaporized.push(vap_astr.clone());
            astroids.remove(&vap_astr);
        }
        station_rot =  calc_angle(&station, &inline[inline.len() - 1]);
    }
    return vaporized;
}

fn solve_puzzle(astroids: &Vec<Position2D>) {

    let mut max_astroids = 0;
    let mut best_place: Option<Position2D> = Option::None;
    for astroid in astroids {
        let count = get_inline(&astroid, astroids).len();
        if count > max_astroids {
            max_astroids = count;
            best_place = Option::from(astroid.clone());
        }
    }

    println!("1.) {}", max_astroids);
    if best_place.is_some() {
        let order = vaporization_procedure(&best_place.unwrap(), astroids);
        if order.len() >= 200 {
            println!("2.) {}", order[199].x * 100 + order[199].y );
        }
    }
}

fn main(){
    title::print_title(10, "Monitoring Station");
    match read_puzzle_input(){
        Ok(res) => solve_puzzle(&res),
        Err(res) => println!("Error while reading puzzle input! {}", res),
    }
}