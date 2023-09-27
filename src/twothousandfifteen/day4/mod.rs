use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: String) -> i32 {
        let mut i = 1;
        loop {
            let s = format!("{}{}", input, i);
            let hash = md5::calculate_hash(&s);
            if hash.starts_with("00000") {
                return i;
            }
            i += 1;
        }
    }

    fn part_two_driver(&self, input: String) -> i32 {
        let mut i = 1;
        loop {
            let s = format!("{}{}", input, i);
            let hash = md5::calculate_hash(&s);
            if hash.starts_with("000000") {
                return i;
            }
            i += 1;
        }
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver{};
        let cases = vec![
            ("abcdef", 609043),
            ("pqrstuv", 1048970),
        ];

        for case in cases {
            assert_eq!(solver.part_one_driver(String::from(case.0)), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), 282749);
    }

    #[test]
    fn part_two_works() {
        // let solver = Solver{};
        // let cases = vec![];

        // for case in cases {
        //     assert_eq!(solver.part_two_driver(String::from(case.0)), case.1, "input = {}", case.0);
        // }

        // assert_eq!(solver.part_two(), 123);
    }
}