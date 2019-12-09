#[path = "../common/title.rs"]
mod title;

use std::fs;

fn split_image(img_str: &str, size_x: u16, size_y: u16) -> Vec<Vec<Vec<char>>> {
    let mut layers = Vec::new();
    let mut index: usize = 0;
    let img_chars: Vec<char> = img_str.chars().collect();
    while index < img_chars.len() {
        let mut columns = Vec::new();
        for _y in 0..size_y {
            let mut rows = Vec::new();
            for _x in 0..size_x {
                rows.push(img_chars[index]);
                index += 1;
            }
            columns.push(rows);
        }
        layers.push(columns);
    }

    return layers;
}

fn check_value(img: &Vec<Vec<Vec<char>>>) -> u32{
    let mut min_zeros = std::u32::MAX;
    let mut check_val: u32 = 0;
    for columns in img {
        let mut count_ones = 0;
        let mut count_twos = 0;
        let mut count_zeros = 0;
        for rows in columns {
            for pixel in rows {
                match *pixel {
                    '0' => count_zeros += 1,
                    '1' => count_ones += 1,
                    '2' => count_twos += 1,
                    _ => {}
                }
            }
        }

        if count_zeros < min_zeros {
            check_val = count_ones * count_twos;
            min_zeros = count_zeros;
        }
    }
    return check_val;
}

fn init_row(row: &mut Vec<char>, len: usize){
    for _i in 0..len{
        row.push('2');
    }
}

fn combine_img(img: &Vec<Vec<Vec<char>>>) -> Vec<Vec<char>>{
    let mut img_combined: Vec<Vec<char>> = Vec::new();
    for layer in 0..img.len() {
        for y in 0..img[layer].len() {

            while img_combined.len() <= y {
                img_combined.push(Vec::new())
            }

            for x in 0..img[layer][y].len() {

                if img_combined[y].len() <= x {
                    init_row(&mut img_combined[y], img[layer][y].len())
                }

                if img_combined[y][x] == '2'{
                    img_combined[y][x] = img[layer][y][x];
                }
            }
        }
    }
    return img_combined;
}

fn print_img(img: &Vec<Vec<char>>){
    let mut img_string: String = String::new();
    for y in 0..img.len() {
        for x in 0..img[y].len() {
            if img[y][x] == '1'{
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

fn main(){
    title::print_title(8, "Space Image Format");
    let img_str = fs::read_to_string("day_08/image.txt").expect("File error!");
    let img = split_image(&img_str, 25, 6);
    println!("1.) {}", check_value(&img));
    println!("2.) < image below >");
    print_img(&combine_img(&img));
}