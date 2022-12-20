use regex::*;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub enum CrateMoverModel {
    CrateMover9000,
    CrateMover9001,
}

pub fn move_crates(file_name: String, crate_mover: CrateMoverModel) -> io::Result<String> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut rev_crate_levels: Vec<String> = vec![];
    let mut crate_levels: Vec<String> = vec![];
    let mut moves: Vec<String> = vec![];
    let mut how_many_crates = 0;
    let mut levels_line: String = "".to_owned();

    reader
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            println!("Parsing line -> {line:?}");
            // load
            if line.contains('[') {
                println!("Pushed line");
                rev_crate_levels.push(line.to_owned());
            } else if line.starts_with(' ') {
                levels_line = line;
                how_many_crates = levels_line
                    .split(' ')
                    .filter(|line| !line.is_empty())
                    .map(|c| c.parse::<i32>().unwrap())
                    .max_by(|x, y| x.cmp(y))
                    .unwrap();
                println!("how_many_crates = {how_many_crates:?}");
            } else if line.starts_with('m') {
                println!("Pushed move");
                moves.push(line.to_owned());
            }
        });

    while let Some(l) = rev_crate_levels.pop() {
        crate_levels.push(l);
    }

    let mut stacks: Vec<Vec<String>> = vec![];

    println!("how_many_crates = {how_many_crates:?}");

    println!("crate_levels len {:?}", crate_levels.len());

    for i in 0..how_many_crates {
        println!("Processing crate {i:?}");
        let mut current_stack: Vec<String> = vec![];
        let mut iter = crate_levels.iter_mut();

        for current_level in iter {
            println!("Finding {i:?} in {current_level:?}");
            let idx = levels_line.find(&(i + 1).to_string()).unwrap();
            println!("Idx for {i} is {idx}");
            let letter = current_level.as_bytes()[idx] as char;
            println!("letter is {letter:}");
            let letter = letter.to_string().replace(' ', "");
            if !letter.to_string().is_empty() {
                current_stack.push(letter.to_string());
            }
        }

        println!("stack is {current_stack:?}");
        stacks.push(current_stack)
    }

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for text in moves {
        let Some(cap) = re.captures(&text) else {
            panic!("Line {text:} is None?");
        };
        let how_many = &cap[1].parse::<usize>().unwrap() + 0;
        // off by one
        let src_stack = &cap[2].parse::<usize>().unwrap() - 1;
        let dst_stack = &cap[3].parse::<usize>().unwrap() - 1;
        println!(
            "{} -> Moving: {} item: from stack {} to stack: {}",
            text,
            how_many,
            (src_stack),
            (dst_stack)
        );

        use CrateMoverModel::*;
        match crate_mover {
            CrateMover9000 => {
                for _ in 0..how_many {
                    let item = stacks[src_stack].pop().unwrap();
                    println!("moving {item:?}");
                    stacks[dst_stack].push(item);
                }
            }
            CrateMover9001 => {
                let mut all_at_once: Vec<String> = vec![];
                for _ in 0..how_many {
                    let item = stacks[src_stack].pop().unwrap();
                    println!("moving {item:?}");
                    all_at_once.push(item);
                }

                while let Some(item) = all_at_once.pop() {
                    stacks[dst_stack].push(item)
                }
            }
        }
    }

    let crates_on_top = stacks
        .iter_mut()
        .map(|s| s.pop())
        .reduce(|acc, letter| Some(acc.unwrap() + &letter.unwrap()));

    let value = crates_on_top.unwrap().unwrap();
    println!("crates_on_top = {value:}");

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let result = move_crates(
            "data/5/tests/input.txt".to_owned(),
            CrateMoverModel::CrateMover9000,
        );
        assert_eq!(result.unwrap(), "CMZ");
    }

    #[test]
    fn part2_works() {
        let result = move_crates(
            "data/5/tests/input.txt".to_owned(),
            CrateMoverModel::CrateMover9001,
        );
        assert_eq!(result.unwrap(), "MCD");
    }
}
