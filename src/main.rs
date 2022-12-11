mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod fs_utils;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // dbg!(args);
    // day1();
    // day2();
    // day3();
    // day4();
    day5();
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

    println!("Total Score with Default Strategy {:?} ", day2::play_tournament(&mut day2::parse(contents.to_owned(), day2::ParsingStrategyType::DefaultStrategy)));
    println!("Total Score with Correct Strategy {:?} ", day2::play_tournament(&mut day2::parse(contents.to_owned(), day2::ParsingStrategyType::CorrectStrategy)));
}

fn day3(){
    header(&"3");
    let file_path = "data/3/input.txt";
    day3::part1(file_path.to_owned());
    day3::part2(file_path.to_owned());
}

fn day4(){
    header(&"4");
    let file_path = "data/4/input.txt";
    day4::part1(file_path.to_owned());
    day4::part2(file_path.to_owned());
}

fn day5(){
    header(&"5");
    let file_path = "data/5/input.txt";
    day5::move_crates(file_path.to_owned(), day5::CrateMoverModel::CrateMover9000);
    day5::move_crates(file_path.to_owned(), day5::CrateMoverModel::CrateMover9001);
    // day4::part2(file_path.to_owned());
}