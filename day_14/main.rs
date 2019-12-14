#[path = "../common/title.rs"]
mod title;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use self::regex::Regex;
use std::collections::HashMap;

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
struct Reaction {
    input: Vec<(String, u32)>,
    output: (String, u32)
}


lazy_static! {
    static ref REACTION_REGEX : Regex = Regex::new(r"(\d+)\s*(\w*)").unwrap();
}

fn parse_ingridient(inp: &str) -> (String, u32){
    let caps = REACTION_REGEX.captures(inp).unwrap();
    return (String::from(caps.get(2).unwrap().as_str()), caps.get(1).unwrap().as_str().parse().unwrap());
}

fn read_puzzle_input() -> std::io::Result<Vec<Reaction>> {
    let file = File::open("day_14/input.txt")?;
    let buffer = BufReader::new(file);
    let mut reactions : Vec<Reaction> = Vec::new();


    for line in buffer.lines() {
        let uline = line.unwrap();
        let parts: Vec<&str> = uline.split("=>").collect();
        
        let unp_ingridients = parts[0];
        let mut ingridients: Vec<(String, u32)> = Vec::new();
        for ingridient in unp_ingridients.split(",") {
            ingridients.push(parse_ingridient(ingridient));
        }

        reactions.push(Reaction {
            input: ingridients,
            output: parse_ingridient(parts[1])
        });
            
    }

    Ok(reactions)
}

fn find_reaction(reactions: &Vec<Reaction>, output: &str) -> usize {
    return reactions.iter().position(|x| x.output.0 == output).unwrap();
}

fn get_ore(reactions: &Vec<Reaction>, output: &str, mut amount: u32, overflow: &mut HashMap<String, u32>) -> u32{

    if *overflow.get(output).unwrap_or(&0) >= amount {
        overflow.insert(output.to_string(), overflow.get(output).unwrap() - amount);
        return 0;
    }

    amount -= *overflow.get(output).unwrap_or(&0);

    let index = find_reaction(reactions, output);
    let reaction = &reactions[index];

    let mut mult = 1;
    while mult * reaction.output.1 < amount {
        mult += 1;
    }

    let mut ore = 0;
    for ingr in &reaction.input {

        if ingr.0 == "ORE" {
            ore += ingr.1 * mult;
            continue;
        }

        let result = get_ore(&reactions, &ingr.0, ingr.1 * mult, overflow);
        ore+= result;
    }
    
    overflow.insert(output.to_string(), mult * reaction.output.1 - amount);
    
    return ore;
}

fn get_ore_once(reactions: &Vec<Reaction>, output: &str) -> f64{
    let index = find_reaction(reactions, output);
    let reaction = &reactions[index];

    let mut ore = 0.0;
    for ingr in &reaction.input {

        if ingr.0 == "ORE" {
            ore += ingr.1 as f64;
            continue;
        }

        ore += get_ore_once(&reactions, &ingr.0) * (ingr.1 as f64);
    }
    
    return ore / (reaction.output.1 as f64);
}

fn solve_puzzle(reactions: &Vec<Reaction>) {
    println!("1.) {}", get_ore(reactions,"FUEL",1,&mut HashMap::new()));
    println!("2.) {}", (1000000000000.0 / get_ore_once(reactions,"FUEL")).floor());
}

fn main(){
    title::print_title(14, "Space Stoichiometry");
    match read_puzzle_input(){
        Ok(res) => solve_puzzle(&res),
        Err(res) => println!("Error while reading puzzle input! {}", res),
    }
}