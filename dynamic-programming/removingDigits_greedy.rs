use std::io;

fn read_number()->i64{
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num:i64 = input.trim().parse().unwrap();
    return num;
}

fn get_digits(n:i64)->Vec<i64>{
    let mut x = n;
    let mut arr:Vec<i64> = Vec::new();
    while x>0{
        let temp = x%10;
        arr.push(temp);
        x/=10;
    }
    return arr;
}

fn min_value(start:&mut i64)->i64{
    let mut steps_count:i64 = 0;
    while *start!=0{
        let digits = get_digits(*start);
        let max_digit = digits.iter().max().unwrap();
        *start -= max_digit;
        steps_count += 1;
    }
    return steps_count;
}
fn main(){
    let mut n = read_number();
    let ans =min_value(&mut n);
    println!("{}",ans);
}   
