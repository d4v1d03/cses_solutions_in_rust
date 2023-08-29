use std::io;

fn read_number()->i64{
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

fn main(){
    let n = read_number();
    let s = n*(n+1)/2;
    if s%2!=0{
        println!("NO");
    }
    else{
        println!("YES");
        let mut subset_sum:i64 = s/2;
        let mut ans:Vec<i64> = Vec::new();
        let mut second_ans:Vec<i64> = Vec::new();
        for i in (1..=n).rev(){
            if i<=subset_sum{
                subset_sum -= i;
                ans.push(i);
            }
        }
        println!("{}",ans.len());
        for x in &ans{
            print!("{} ",x);
        }
        println!("");
        ans.sort();
        for i in 1..=n{
            let result = ans.binary_search(&i);
            match result{
                Ok(_)=>{
                },
                Err(_)=>{
                    second_ans.push(i);
                }
            }
        }
        println!("{}",second_ans.len());
        for i in second_ans{
            print!("{} ",i);
        }
    }
}   
