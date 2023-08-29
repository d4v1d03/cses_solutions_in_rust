use std::{io, collections::{BTreeSet}};

fn read_string()->String{
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return String::from(input.trim());
}

fn generate_strings(string_store:&mut BTreeSet<String>,low:usize,high:usize,input:&mut Vec<char>){
    if low==high{
        string_store.insert(input.iter().collect::<String>());
    }
    for i in low..=high{
        input.swap(low,i);
        generate_strings(string_store,low+1,high,input);
        input.swap(low,i);
    }
}
fn main(){
    let input:String = read_string();
    let mut string_store:BTreeSet<String> = BTreeSet::new();
    let low:usize = 0;
    let high:usize = input.len() -1;
    let mut input_vec:Vec<char> = input.chars().collect();   

    generate_strings(&mut string_store, low, high,&mut input_vec);

    println!("{}",string_store.len());
    for item in string_store{
        println!("{}",item);
    }
}
