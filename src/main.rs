mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod fs_utils;

#[allow(unused_macros)]
fn main() {
    // let args: Vec<String> = env::args().collect();
    // dbg!(args);
    day1();
    day2();
    day3();
    day4();
    day5();
    day6()
}

fn header(day: &str){
    println!("Day {:?} ============", day)
}

fn day1(){
    header("1");
    let file_path = "data/1/input.txt";
    println!("In file {}", file_path);
    let contents = fs_utils::read(file_path.to_owned());
    let elves = day1::parse(contents);
    println!("Elf with most is {:?} ", day1::who_most(&elves));
    println!("Top 3 Elves with most is {:?} ", day1::top_three(&elves));
}

fn day2(){
    header("2");
    let file_path = "data/2/input.txt";
    let contents = fs_utils::read(file_path.to_owned());

    println!("Total Score with Default Strategy {:?} ", day2::play_tournament(&mut day2::parse(contents.to_owned(), day2::ParsingStrategyType::DefaultStrategy)));
    println!("Total Score with Correct Strategy {:?} ", day2::play_tournament(&mut day2::parse(contents, day2::ParsingStrategyType::CorrectStrategy)));
}

fn day3(){
    header("3");
    let file_path = "data/3/input.txt";
    let priority_sum = day3::part1(file_path.to_owned());
    println!("Priority Sum is {:?}", priority_sum);

    let priority_sum = day3::part2(file_path.to_owned());
    println!("Priority Sum is {:?}", priority_sum);
}

fn day4(){
    header("4");
    let file_path = "data/4/input.txt";
    let how_many_pairs = day4::part1(file_path.to_owned());
    println!("how_many_pairs = {how_many_pairs:?}");

    let how_many_pairs = day4::part2(file_path.to_owned());
    println!("how_many_pairs = {how_many_pairs:?}");
}

fn day5(){
    header("5");
    let file_path = "data/5/input.txt";
    let value = day5::move_crates(file_path.to_owned(), day5::CrateMoverModel::CrateMover9000);
    println!("crates_on_top = {value:?}");

    let value = day5::move_crates(file_path.to_owned(), day5::CrateMoverModel::CrateMover9001);
    println!("crates_on_top = {value:?}");

    // day4::part2(file_path.to_owned());
}

fn day6(){
    header("6");
    day6::day6_main();
}