use std::io;
fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut n:i64 = input.trim().parse().expect("Invalid number");

    while n!=1{
        print!("{} ",n);
        if n&1==0 {
            n /= 2;
        }
        else{
            n = n*3 + 1;
        }
    }
    print!("1");
}
