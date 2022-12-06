use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn parse(file_name: String) -> io::Result<()>  {
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
            for c in  chars1{
                if hash.contains(&c) {
                    return c;
                } else{
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
            println!("Processing letter {:?}, {:?}", letter, ( value + 1));
            return acc + value + 1
            // return acc + (letter as u32)
            // acc
        })
    ;
    println!("Priority Sum is {:?}",priority_sum);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_works() {
        println!("b is {:?}, {:?}",  'Z' as u32  - 'A' as u32 , 'z'.is_uppercase());
        parse("data/3/tests/input.txt".to_owned());
    }
}
