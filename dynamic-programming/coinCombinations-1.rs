use std::io;

const MOD:i64 = 1000000007;
fn read_vector()->Vec<i64>{
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let arr:Vec<i64> = input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
    return arr;
}

fn count_ways(coins:&Vec<i64>,x:i64,dp:&mut Vec<i64>)->i64{
    if x==0{
        return 1;
    }
    if dp[x as usize] != -1{
        return dp[x as usize]
    }
    let mut ans:i64 = 0;
    for coin in coins{
        if x>=*coin{
            ans += count_ways(coins,x-coin,dp)%MOD;
        }
    }
    dp[x as usize] = ans%MOD;
    return ans%MOD;
}
fn main(){
    let line_one = read_vector();
    let n = line_one[0];
    let x = line_one[1];
    let mut dp:Vec<i64> = Vec::new();
    dp.resize(x as usize + 1,-1);
    let coins = read_vector();
    let ans = count_ways(&coins,x,&mut dp)%MOD;
    println!("{}",ans);
}
