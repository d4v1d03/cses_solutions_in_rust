use std::io;

fn read_number()->u32{
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

fn read_vector()->Vec<i64>{
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let arr = input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
    return arr;
}

fn main(){
    let n:u32 = read_number();
    let weights:Vec<i64> = read_vector();
    let s:i64 = weights.iter().sum();
    let pown = 2_i64.pow(n);
    let mut curr:i64 = i64::MAX;

    for i in 0..pown{
        let mut comb:Vec<i64> = Vec::new();
        for j in 0..n{
            if i&(1<<j)!=0{
                comb.push(weights[j as usize]);
            }
        }
        let s2:i64 = comb.iter().sum();
        let diff = s-2*s2;
        if diff>=0{
            curr = std::cmp::min(curr,diff);
        }
    }
    println!("{}",curr);
}
