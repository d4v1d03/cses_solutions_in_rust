use std::io;
fn read_vector() -> Vec<i64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    return arr;
}

fn get_min_coins(x:i64,coins:&Vec<i64>,dp:&mut Vec<i64>)->i64{
    if x==0{
        return 0;
    }
    if dp[x as usize]!=-1{
        return dp[x as usize];
    }
    let mut min_value = i64::MAX;

    for coin in coins{
        if x>=*coin{
            let curr_value = get_min_coins(x-coin,coins,dp);
            if curr_value != i64::MAX{
                min_value = std::cmp::min(min_value,curr_value+1);
            }
        }
    }
    dp[x as usize] = min_value;
    return min_value;
}
fn main() {
    let first_line = read_vector();
    let n = first_line[0];
    let x = first_line[1];
    let coin_values = read_vector();

    let mut dp:Vec<i64> = Vec::new();
    dp.resize(x as usize+1, -1);

    let ans = get_min_coins(x, &coin_values,&mut dp);
    if ans!=i64::MAX{
        println!("{}", ans);
    }
    else{
        println!("{}",-1);
    }
}

