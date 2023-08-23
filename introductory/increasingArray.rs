use std::io;

fn read_number()->usize{
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read string");
    let num:usize = input.trim().parse().expect("Failed to read num");
    return num;
}

fn read_vector() -> Vec<usize> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<usize> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    return arr;
}

fn main(){
    let n:usize = read_number();
    let mut nums:Vec<usize> = read_vector();
    let mut count_moves:usize = 0;

    for i in 1..n{
        if nums[i]<nums[i-1]{
            count_moves += nums[i-1]-nums[i];
            nums[i] = nums[i-1];
        }
    }
    println!("{}",count_moves);
}
