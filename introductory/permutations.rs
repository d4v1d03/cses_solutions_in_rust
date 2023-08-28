use std::io;

fn read_number()->i64{
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read string");
    let num:i64 = input.trim().parse().expect("Failed to read num");
    return num;
}

fn read_vector() -> Vec<usize> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<usize> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    return arr;
}

fn main(){
    let n:i64 = read_number();
    let mut vec1:Vec<i64> = Vec::with_capacity(n as usize);
    let mut vec2:Vec<i64> = Vec::with_capacity(n as usize);

    if n<=3 && n>1{
        println!("NO SOLUTION");
    }
    else if n==4{
        println!("2 4 1 3");
    }
    else{
        let mut i:i64 = n;
        while i>=1{
            vec1.push(i);
            if i-1!=0{
                vec2.push(i-1);
            }
            i -= 2;
        }
        for j in vec1{
            print!("{} ",j);
        }
        for j in vec2{
            print!("{} ",j);
        }
    }
}
