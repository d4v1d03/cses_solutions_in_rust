use std::io;

fn read_number() -> i64 {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num: i64 = input.trim().parse().unwrap();
    return num;
}

fn get_digits(n: i64) -> Vec<i64> {
    let mut x = n;
    let mut arr: Vec<i64> = Vec::new();
    while x > 0 {
        let temp = x % 10;
        arr.push(temp);
        x /= 10;
    }
    return arr;
}

fn min_value(start: i64) -> i64 {
    let mut dp:Vec<i64> = vec![i64::MAX-1;start as usize+1];
    dp[0] = 0;
    for i in 0..=start{
        let digits = get_digits(i);
        for digit in digits{
            dp[i as usize] = std::cmp::min(dp[i as usize],dp[(i-digit)as usize]+1);
        }
    }
    return dp[start as usize];
}
fn main() {
    let mut n = read_number();
    let ans = min_value(n);
    println!("{}", ans);
}

