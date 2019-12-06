use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;


#[derive(Clone, Debug)]
struct OrbitMap {
    full_map: HashMap<String, Body>,
    roots: Vec<String>
}

#[derive(Clone, Debug)]
struct Body {
    sub: Vec<String>,
    parent: String
}


fn read_puzzle_input() -> std::io::Result<HashMap<String, Body>> {
    let file = File::open("day_06/input.txt")?;
    let buffer = BufReader::new(file);
    let mut bodies : HashMap<String, Body> = HashMap::new();

    for line in buffer.lines() {
        let uline = line.unwrap();
        let parts: Vec<&str> = uline.split(")").collect();
        let main_key = String::from(parts[0]);

        let mut main_b = bodies.get_mut(&main_key);
        if main_b.is_none() {
            bodies.insert(main_key.clone(), Body {
                sub: Vec::new(),
                parent: String::from("")
            });
            main_b = bodies.get_mut(&main_key);
        }

        main_b.unwrap().sub.push(String::from(parts[1]));
    }

    Ok(bodies)
}


fn map_depths(map: & HashMap<String, Body>) -> OrbitMap{
    let mut dmap = map.clone();
    let mut roots : Vec<String> = Vec::new();
    for item in map.into_iter(){
        let mut is_sub = false;
        for item2 in map.into_iter(){
            if (&item2.1.sub).into_iter().find(|&x| x == item.0).is_some() {
                is_sub = true;
                dmap.get_mut(item.0).unwrap().parent = item2.0.clone();
                break;
            }
        }

        if !is_sub {
            roots.push(item.0.clone());
        }
    }

    let mut omap = OrbitMap {
        full_map: dmap,
        roots: roots
    };

    for root in omap.roots.clone() {
        put_ends_recursive(&mut omap, &root);
    }

    return omap;
}

fn put_ends_recursive(map:& mut OrbitMap, node_name: & String) {

    let fmap = map.full_map.clone();
    let node = fmap.get(node_name).unwrap();

    for sub in &node.sub{
        let sub_node = fmap.get(sub);
        if sub_node.is_some() {
            put_ends_recursive(map, &sub);
        }
        else {
            map.full_map.insert(sub.clone(), Body {
                sub: Vec::new(),
                parent: node_name.clone()
            });
        }
    }
}

fn count_orbits(map:& OrbitMap, root:& String, depth: u32) -> u32{
    let mut orbits: u32 = depth;
    let opt_body = map.full_map.get(root);

    if opt_body.is_some() {
        for sub in &opt_body.unwrap().sub{
            orbits += count_orbits(map, sub, depth + 1);
        }
    }
    
    return orbits;
}

fn get_path(map:& OrbitMap, node: &String) -> Vec<String>{
    let mut path: Vec<String> = Vec::new();
    let node_r = &map.full_map.get(node).unwrap();
    if node_r.parent.len() == 0 {
        return vec![node.clone()];
    }
    path.extend(get_path(map, &node_r.parent));
    path.push(node.clone());
    return path;
}

fn get_hops(map:& OrbitMap, node_a: &String, node_b: &String) -> usize{
    let path_a = get_path(map, node_a);
    let path_b = get_path(map, node_b);
    let min_len = usize::min(path_a.len(), path_b.len());
    let mut common = 0;
    for i in 0..min_len {
        if path_a[i] != path_b[i]{
            common = i;
            break;
        }
    }
    return (path_a.len() - common) + (path_b.len() - common) - 2;
}

fn solve_puzzle(res: HashMap<String, Body>){
    let map = map_depths(&res);
    let mut orbits : u32 = 0;
    for item in &map.roots{
        orbits += count_orbits(&map, &item, 0);
    }

    println!("1.) {}", orbits);
    println!("2.) {}", get_hops(&map, &String::from("YOU"), &String::from("SAN")));
}

fn main(){
    match read_puzzle_input(){
        Ok(res) => solve_puzzle(res),
        Err(res) => println!("Error while reading puzzle input! {}", res),
    }
}