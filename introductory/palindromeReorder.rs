use std::{io, collections::HashMap};

fn read_string()->String{
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return String::from(input.trim());
}
fn main(){
    let inp = read_string();
    let mut hmap:HashMap<char,i32> = HashMap::new();

    for c in inp.chars(){
        let curr = hmap.entry(c).or_insert(0);
        *curr += 1;
    }

    let mut odd_ctr = 0;
    let mut odd_char = ' ';

    for item in &hmap{
        if item.1 & 1 != 0{
            odd_char = *item.0;
            odd_ctr += 1;
            if odd_ctr >1 {
                println!("NO SOLUTION");
                return;
            }
        }
    }
    
    let mut part1:String = String::new();
    let mut part2:String = String::new();

    for item in hmap{
        part1 += (0..item.1/2).map(|_|item.0).collect::<String>().as_str();
    }
    part2 = part1.chars().rev().collect::<String>();

    let mut answer:String = String::new();
    if odd_ctr==1{
        part1.push(odd_char);
    }
    answer = part1 + part2.as_str();
    println!("{}",answer);

}
