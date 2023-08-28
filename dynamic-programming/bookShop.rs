use std::io;

fn read_vector() -> Vec<i64> {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    return arr;
}

fn get_max_pages(price: &Vec<i64>, pages: &Vec<i64>, n: i64, x: i64, dp: &mut Vec<Vec<i64>>) -> i64 {
    if n == 0 || x == 0 {
        return 0;
    }
    if dp[n as usize - 1][x as usize] != -1 {
        return dp[n as usize - 1][x as usize];
    }
    if price[n as usize - 1] <= x {
        let y = std::cmp::max(
            pages[n as usize - 1] + get_max_pages(price, pages, n - 1, x - price[n as usize - 1], dp),
            get_max_pages(price, pages, n - 1, x, dp),
        );
        dp[n as usize - 1][x as usize] = y;
        return y;
    }
    let y = get_max_pages(price, pages, n - 1, x, dp);
    dp[n as usize - 1][x as usize] = y;
    return y;
}

fn main() {
    let first_line: Vec<i64> = read_vector();
    let price: Vec<i64> = read_vector();
    let pages: Vec<i64> = read_vector();
    let n = first_line[0];
    let x = first_line[1];
    let mut dp:Vec<Vec<i64>> = vec![vec![-1;x as usize+1];n as usize+1];
    let ans = get_max_pages(&price, &pages, n, x,&mut dp);
    println!("{}",ans);
}

