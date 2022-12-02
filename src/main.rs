mod parser;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let file_path = &args[1];
    
    println!("In file {}", file_path);
    let elves = parser::parse(parser::read(file_path.to_owned()));
    // println!("Elf with most is {:?} ", parser::who_most(elves));

    println!("Top 3 Elves with most is {:?} ", parser::top_three(elves));

}
