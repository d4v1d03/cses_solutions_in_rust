use std::io;

fn read_num()->i64{
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read string");
    let num:i64 = input.trim().parse().expect("Failed to read number");
    return num;
}

fn read_vector() -> Vec<i64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i64> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    return arr;
}


fn get_sum_to_n(num:i64)->i64{
    return num*(num+1)/2;
}
fn main(){
    let n:i64 = read_num();
    let mut sum:i64 = 0;
    let arr:Vec<i64> = read_vector();

    for i in 0..n-1{
        sum += arr[i as usize];
    }
    
    let sum_to_n:i64 = get_sum_to_n(n);

    println!("{}",sum_to_n-sum);
}
