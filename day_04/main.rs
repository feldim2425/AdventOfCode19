const INPUT_LOWER : u32 = 158126;
const INPUT_UPPER : u32 = 624574;


fn decompose_number(mut input: u32) -> Vec<u8>{
    let mut digits: Vec<u8> = Vec::new(); 
    while input >= 1{
        digits.push(((input % 10) as u8).into());
        input /= 10;
    }
    return digits;
}

fn increace_number(num:& mut Vec<u8>){
    let mut i :usize = 0;
    loop {
        num[i] += 1;
        if num[i] >= 10 {
            num[i] -= 10;
            i += 1;
        }
        else {
            break;
        }
    }
}

fn reached_limit(num:&Vec<u8>, max:&Vec<u8>) -> bool{
    if num.len() < max.len(){
        return false;
    }
    else if num.len() > max.len(){
        return true;
    }

    for i in (0..num.len()).rev(){
        if max[i] < num[i]{
            return true;
        }
        else if max[i] > num[i]{
            return false;
        }
    }
    return false;
}

fn check_number(num: &Vec<u8>) -> u8{
    let mut last_digit: u8 = 10;
    let mut streak: u8 = 0;
    let mut min_streak: u8 = 10;
    for dig in num {
        if *dig > last_digit {
            return 0;
        }

        if *dig == last_digit {
            streak += 1;
        }
        else if streak != 0 && streak < min_streak{
            min_streak = streak;
            streak = 0;
        }
        last_digit = *dig;
    }

    if streak != 0 && streak < min_streak{
        min_streak = streak;
    }

    if min_streak == 10{
        return 0;
    }

    return min_streak;
}

fn solve_puzzle(){
    let mut num = decompose_number(INPUT_LOWER);
    let max = decompose_number(INPUT_UPPER);
    let mut count  = 0;
    let mut count2  = 0;
    while !reached_limit(&num, &max){
        let streak = check_number(&num);
        if streak >= 1{
            count += 1;
        }
        if streak == 1 {
            count2 += 1;
        }

        increace_number(&mut num)
    }
    

    println!("1.) {}", count);
    println!("2.) {}", count2);
}

/*fn test_number(num: u32) -> u8{
    let v = decompose_number(num);
    return check_number(&v);
}*/

fn main(){
    solve_puzzle();
    /*println!("{}", test_number(111111));
    println!("{}", test_number(223450));
    println!("{}", test_number(123789));
    println!("{}", test_number(112233));
    println!("{}", test_number(123444));
    println!("{}", test_number(111122));*/
}