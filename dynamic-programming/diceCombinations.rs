use std::{ io};

const MOD:i64 = 1000000007;

fn read_number() -> i64 {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num: i64 = input.trim().parse().unwrap();
    return num;
}

fn dice_combinations(n:i64,dp:&mut Vec<i64>)->i64{
    if n==0{
        return 0;
    }
    if n==1{
        return 1;
    }
    if dp[n as usize -1]!=-1{
        return dp[n as usize -1];
    }
    let mut ans = 0;
    for i in 1..=6{
        if n-i>=0{
            ans += dice_combinations(n-i,dp)%MOD;
        }
    }
    dp[n as usize -1 ] = ans;
    return ans%MOD;
}

fn main() {
    let target:i64 = read_number();
    let mut dp:Vec<i64> = Vec::new();
    dp.resize(target as usize+1,-1);
    let ans = dice_combinations(target+1,&mut dp)%MOD;
    println!("{}",ans);
}



