use std::io;
fn read_number()->i64{
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num:i64 = input.trim().parse().unwrap();
    return num;
}     
fn main(){
    let n = read_number();
    let mut ctr = 1;
    let mut ans = 0;

    while true{
        let temp:i64 = n/(5*ctr);
        if temp==0{
            break;
        }
        ans += temp;
        ctr *= 5;
    }
    println!("{}",ans);
}
