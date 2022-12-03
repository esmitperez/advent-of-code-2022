mod day1;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let file_path = &args[1];
    
    println!("In file {}", file_path);
    let elves = day1::parse(day1::read(file_path.to_owned())).to_owned();
    println!("Elf with most is {:?} ", day1::who_most(&elves));
    println!("Top 3 Elves with most is {:?} ", day1::top_three(&elves));

}
