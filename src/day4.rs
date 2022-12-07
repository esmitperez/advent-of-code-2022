use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn part1(file_name: String) -> io::Result<usize> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let how_many_pairs = reader
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap())
        .map(|line| {
            let ranges = line.split(",").collect::<Vec<&str>>();
            let range1 = ranges[0].split("-").collect::<Vec<&str>>();;
            let range2 =  ranges[1].split("-").collect::<Vec<&str>>();;

            let r1_start = range1[0].parse::<i32>().unwrap();
            let r1_end = range1[1].parse::<i32>().unwrap();

            let r2_start = range2[0].parse::<i32>().unwrap();
            let r2_end = range2[1].parse::<i32>().unwrap();

            println!("Evaluating {r1_start:?}-{r1_end:?}, {r2_start:?}-{r2_end:?}");
        
            if (r2_start <= r1_start && r2_end >= r1_end) || (r1_start <= r2_start && r1_end >= r2_end) {
                println!("YES");
                Ok(())
            } else {
                println!("NO");
                Err(())
            }
        })
        .filter(|x| x.is_ok())
        .count();

        println!("how_many_pairs = {how_many_pairs:}");
        
    Ok(how_many_pairs)
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn part1_works() {
        let result = part1("data/4/tests/input.txt".to_owned());
        assert_eq!(result.unwrap(), 2);
    }
}