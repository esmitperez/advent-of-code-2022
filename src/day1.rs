use std::collections::HashMap;
use std::collections::BinaryHeap;


pub fn parse(contents: String) -> HashMap<String, u32> {
    let mut scores = HashMap::new();

    let lines = contents.split("\n");

    let mut k = 1;
    let mut v:u32 = 0;
    'again: for l in lines {
        // println!("Parsing line {:?}, {:?}", k, l);

        if l.is_empty() {
            k = k+1;
            v = 0;
            continue 'again;
        }

        // dbg!(k);
        
        let qty:u32 = l.parse().unwrap();
        v = v + qty;

        scores.insert(k.to_string(), v);
    }

    scores
}

pub fn who_most(elves: &HashMap<String, u32>) -> u32 {
    let mut most:u32 = 0;

    for (_k,v) in elves{
        if v > &most {
            most = *v;
        }
    }

    most
}

pub fn top_three(elves: &HashMap<String, u32>) -> u32 {

    let mut heap:BinaryHeap<u32> = BinaryHeap::new();

    for (_k, v) in elves {
        heap.push(*v);
    }

    let mut total = 0;

    for _i in 0..3 {
        match heap.pop() {
            Some(v) => {
                total = total + v;
            },
            None => {}
        }
    }

    total
}


#[cfg(test)]
mod tests {
    use super::*;
    const EXPECTED:&str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

    #[test]
    fn parse_works() {
        let result = parse(EXPECTED.to_owned());
        dbg!(&result);
        let r = result.get("1").unwrap_or(&0);
        assert_eq!(r, &6000);
        assert_eq!(result.len(),5);

        dbg!(&result);
        for (key, value) in &result {
            println!("{}: {}", key, value);
        }
    }

    #[test]
    fn who_most_works(){
        let most = who_most(&parse(EXPECTED.to_owned()));
        assert_eq!(most,24000);
    }

    #[test]
    fn top_three_works(){
        let most = top_three(&parse(EXPECTED.to_owned()));
        assert_eq!(most,45000);
    }
}