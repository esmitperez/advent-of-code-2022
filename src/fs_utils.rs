
pub fn read(file_path: String) -> String {
    let contents =
        std::fs::read_to_string(file_path).expect("Should have been able to read the file");
    contents
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXPECTED: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

    #[test]
    fn read_works() {
        let result = read("data/1/tests/input.txt".to_owned());
        assert_eq!(result, EXPECTED);
    }
}
