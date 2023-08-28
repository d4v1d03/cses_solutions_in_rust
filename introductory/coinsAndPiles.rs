use std::io;
fn read_number()->i64{
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num:i64 = input.trim().parse().unwrap();
    return num;
}     

fn read_vector()->Vec<i64>{
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let arr = input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
    return arr;
}


fn main(){
    let t = read_number();

    for _ in 0..t{
        let ab_vec:Vec<i64> = read_vector();
        let a = ab_vec.iter().max().unwrap();
        let b = ab_vec.iter().min().unwrap();
        let x = 2*a - b;
        let y = 2*b - a;
        if x%3==0 && y%3==0 && x>=0 && y>=0{
            println!("YES");
        }
        else{
            println!("NO");
        }
    }
}

