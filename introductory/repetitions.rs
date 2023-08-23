use std::{io,cmp};

fn main(){
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.len();
    let mut cnt = 1;
    let mut cur_max = i64::MIN;
    let bts = input.as_bytes();

    for i in 0..n-1{
        if bts[i]==bts[i+1]{
            cnt +=1 ;
        }
        else{
            cur_max = cmp::max(cur_max,cnt);
            cnt = 1;
        }
    }
    println!("{}",cur_max);
}
