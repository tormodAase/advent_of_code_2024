use regex::Regex;

use crate::helpers::Solver;

#[derive(Clone, Debug, PartialEq)]
pub struct DayThreePuzzleOne {
    number_pairs: Vec<(usize, usize)>,
}

impl Solver for DayThreePuzzleOne {
    fn new() -> Self
    where
        Self: Sized,
    {
        DayThreePuzzleOne {
            number_pairs: vec![],
        }
    }
    fn parse(&mut self, string: &str) -> () {
        println!("regex");
        let regex = Regex::new(r"(?:mul)[(](\d{1,3})[,](\d{1,3})[)]").unwrap();
        println!("IS MATCH: {}", regex.is_match(string));
        for (s, [d1, d2]) in regex.captures_iter(string).map(|c| c.extract()) {
            println!("{}", s);
            self.number_pairs
                .push((d1.parse().unwrap(), d2.parse().unwrap()));
        }
    }

    fn solve(&mut self) -> String {
        self.number_pairs
            .iter()
            .fold(0, |acc, val| acc + (val.0 * val.1))
            .to_string()
    }
}

#[test]
fn test_parse() {
    let mut solver = DayThreePuzzleOne::new();

    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    let expected_pairs = vec![(2, 4), (5, 5), (11, 8), (8, 5)];
    solver.parse(input);
    assert_eq!(
        solver,
        DayThreePuzzleOne {
            number_pairs: expected_pairs
        }
    )
}

#[test]
fn test_solve() {
    let mut solver = DayThreePuzzleOne {
        number_pairs: vec![(2, 4), (5, 5), (11, 8), (8, 5)],
    };

    assert_eq!(solver.solve(), "161");
}
