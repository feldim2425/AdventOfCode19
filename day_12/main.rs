#[path = "../common/title.rs"]
mod title;

#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate num;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use self::regex::Regex;
use std::collections::HashSet;

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
struct Vec3D {
    x: i16,
    y: i16,
    z: i16,
}

impl Vec3D {
    fn zero() -> Self{
        return Vec3D {
            x: 0,
            y: 0,
            z: 0
        }
    }
}

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
struct Moon {
    velocity: Vec3D,
    pos: Vec3D,
}

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
struct Moon1D {
    velocity: i16,
    pos: i16,
}

impl Moon {
    fn energy(self: &Self) -> i32{
        let kin = self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs();
        let pot = self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs();
        return kin as i32 * pot as i32;
    }
    
}

lazy_static! {
    static ref CABELE_REGEX : Regex = Regex::new(r"(?m)<x=(?P<x>-?\d*),\s*y=(?P<y>-?\d*),\s*z=(?P<z>-?\d*)>").unwrap();
}

fn read_puzzle_input() -> std::io::Result<Vec<Vec3D>> {
    let file = File::open("day_12/input.txt")?;
    let buffer = BufReader::new(file);
    let mut moon_pos : Vec<Vec3D> = Vec::new();


    for line in buffer.lines() {
        let uline = line.unwrap();
        let caps = CABELE_REGEX.captures(&uline).unwrap();
        moon_pos.push(Vec3D {
            x: caps.name("x").map_or(0, |m| m.as_str().parse().unwrap()),
            y: caps.name("y").map_or(0, |m| m.as_str().parse().unwrap()),
            z: caps.name("z").map_or(0, |m| m.as_str().parse().unwrap()),
        })
            
    }

    Ok(moon_pos)
}

fn step_position(moons: &mut Vec<Moon>){
    let temp = moons.clone();
    for moon in moons {
        for temp_moon in &temp {
            if temp_moon.pos.x < moon.pos.x {
                moon.velocity.x -= 1;
            }
            else if temp_moon.pos.x > moon.pos.x {
                moon.velocity.x += 1;
            }

            if temp_moon.pos.y < moon.pos.y {
                moon.velocity.y -= 1;
            }
            else if temp_moon.pos.y > moon.pos.y {
                moon.velocity.y += 1;
            }

            if temp_moon.pos.z < moon.pos.z {
                moon.velocity.z -= 1;
            }
            else if temp_moon.pos.z > moon.pos.z {
                moon.velocity.z += 1;
            }
        }

        moon.pos.x += moon.velocity.x;
        moon.pos.y += moon.velocity.y;
        moon.pos.z += moon.velocity.z;
    }
}

fn step_position_1d(moons: &mut Vec<Moon1D>){
    let temp = moons.clone();
    for moon in moons {
        for temp_moon in &temp {
            if temp_moon.pos < moon.pos {
                moon.velocity -= 1;
            }
            else if temp_moon.pos > moon.pos {
                moon.velocity += 1;
            }
        }

        moon.pos += moon.velocity;
    }
}

fn find_steps_1d(moons: &mut Vec<Moon1D>) -> u64{
    let mut states: HashSet<Vec<Moon1D>> = HashSet::new();
    while states.get(moons).is_none() {
        states.insert(moons.clone());
        step_position_1d(moons);
    }
    return states.len() as u64;
}


fn solve_puzzle(moon_pos: &Vec<Vec3D>) {
    let mut moons: Vec<Moon> = Vec::new(); 
    let mut moons1d: Vec<Vec<Moon1D>> = Vec::new();
    
    moons1d.push(Vec::new());
    moons1d.push(Vec::new());
    moons1d.push(Vec::new());

    for pos in moon_pos {
        moons.push(Moon {
            pos: *pos,
            velocity: Vec3D::zero()
        });

        moons1d[0].push(Moon1D {
            pos: pos.x,
            velocity: 0
        });
        moons1d[1].push(Moon1D {
            pos: pos.y,
            velocity: 0
        });
        moons1d[2].push(Moon1D {
            pos: pos.z,
            velocity: 0
        });
    }
    for _i in 0..1000 {
        step_position(&mut moons);
    }

    let mut energy = 0;
    for moon in &moons {
        energy += moon.energy();
    }
    println!("1.) {}", energy);

    let step_1 = find_steps_1d(&mut moons1d[0]);
    let step_2 = find_steps_1d(&mut moons1d[1]);
    let step_3 = find_steps_1d(&mut moons1d[2]);
    println!("2.) {}", num::integer::lcm(step_1, num::integer::lcm(step_2, step_3)));
}

fn main(){
    title::print_title(12, "The N-Body Problem");
    match read_puzzle_input(){
        Ok(res) => solve_puzzle(&res),
        Err(res) => println!("Error while reading puzzle input! {}", res),
    }
}