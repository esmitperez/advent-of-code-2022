use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn part1(file_name: String) -> io::Result<u32> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let priority_sum = reader
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap())
        .map(|line| {
            let middle = (line.len() / 2);
            let (compartment1, compartment2) = line.split_at(middle);
            let mut chars1 = compartment1.chars().collect::<Vec<char>>();
            let mut chars2 = compartment2.chars().collect::<Vec<char>>();
            // println!("C1 {:?}", chars1);
            // println!("C2 {:?}", chars2);

            chars1.sort();
            chars1.dedup();

            chars2.sort();
            chars2.dedup();

            chars1.append(&mut chars2);

            let mut hash = HashSet::<char>::new();
            for c in chars1 {
                if hash.contains(&c) {
                    return c;
                } else {
                    hash.insert(c);
                }
            }
            '-'
        })
        .fold(0, |acc, letter| {
            let value = if letter.is_uppercase() {
                letter as u32 - 'A' as u32 + 26
            } else {
                letter as u32 - 'a' as u32
            };
            println!("Processing letter {:?}, {:?}", letter, (value + 1));
            return acc + value + 1;
            // return acc + (letter as u32)
            // acc
        });
    println!("Priority Sum is {:?}", priority_sum);

    Ok(priority_sum)
}

pub fn part2(file_name: String) -> io::Result<u32> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let contents = reader
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let mut groups: Vec<Vec<&String>> = vec![];

    let mut iter = contents.iter();
    let l = &contents.len();
    let mut counter = 0;

    while &counter < l {
        let mut g: Vec<&String> = vec![
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        ];

        counter += 3;
        groups.push(g);
    }

    let mut total = 0;

    let priority_sum = groups
        .iter()
        .map(|g| {
            let mut group_chars = "".to_string().chars().collect::<Vec<char>>();

            let mut g_iter = g.iter();

            let x = &mut g_iter.next().unwrap().chars().collect::<Vec<char>>();
            x.sort();
            x.dedup();
            group_chars.append(x);

            let x = &mut g_iter.next().unwrap().chars().collect::<Vec<char>>();
            x.sort();
            x.dedup();
            group_chars.append(x);

            let x = &mut g_iter.next().unwrap().chars().collect::<Vec<char>>();
            x.sort();
            x.dedup();
            group_chars.append(x);

            group_chars.sort();

            println!("Group of 3 is {group_chars:?}");

            let mut last_letter = '-';
            let mut counter = 0;
            for c in group_chars {
                if last_letter != c {
                    last_letter = c;
                    counter = 1;
                } else {
                    counter += 1;
                }
                if counter == 3 {
                    return c;
                }
            }
            '-'
        })
        .fold(0, |acc, letter| {
            println!("Processing letter {:?}", letter);

            let value = if letter.is_uppercase() {
                letter as u32 - 'A' as u32 + 26
            } else {
                letter as u32 - 'a' as u32
            };
            println!("Processing letter {:?}, {:?}", letter, (value + 1));
            acc + value + 1
            // return acc + (letter as u32)
            // acc
        });

    println!("Groups priority_sum {:?}", priority_sum);

    Ok(priority_sum)
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_works() {
        println!(
            "b is {:?}, {:?}",
            'Z' as u32 - 'A' as u32,
            'z'.is_uppercase()
        );
        let result = part1("data/3/tests/input.txt".to_owned());
        assert_eq!(result.unwrap(), 157);
    }

    #[test]
    fn part2_works() {
        println!(
            "b is {:?}, {:?}",
            'Z' as u32 - 'A' as u32,
            'z'.is_uppercase()
        );
        let result = part2("data/3/tests/input.txt".to_owned());
        assert_eq!(result.unwrap(), 70);
    }
}
