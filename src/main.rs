mod day1;
mod day2;
mod fs_utils;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // dbg!(args);
    day1();
    day2()
}

fn header(day: &str){
    println!("Day {:?} ============", day)
}

fn day1(){
    header(&"1");
    let file_path = "data/1/input.txt";
    println!("In file {}", file_path);
    let contents = fs_utils::read(file_path.to_owned());
    let elves = day1::parse(contents).to_owned();
    println!("Elf with most is {:?} ", day1::who_most(&elves));
    println!("Top 3 Elves with most is {:?} ", day1::top_three(&elves));
}

fn day2(){
    header(&"2");
    let file_path = "data/2/input.txt";
    let contents = fs_utils::read(file_path.to_owned());
    
    
    println!("Total Score as per guide {:?} ", day2::play_tournament(&mut day2::parse(contents.to_owned())));
}
