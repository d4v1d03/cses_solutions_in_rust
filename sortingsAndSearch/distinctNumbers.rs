use std::{io,collections::HashSet};

fn read_number()->i64{
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn read_vector()->Vec<i64>{
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let arr:Vec<i64> = input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
    return arr;
}
fn main(){
    let n:i64 = read_number();
    let values:Vec<i64> = read_vector();
    let mut set:HashSet<i64> = HashSet::new();
    for value in values{
        set.insert(value);
    }
    println!("{}",set.len());
}
