use crate::helpers::*;
use std::vec;

#[derive(PartialEq, Debug, Clone)]
pub struct DayOnePuzzleOne {
    list_one: Vec<usize>,
    list_two: Vec<usize>,
}

impl Solver for DayOnePuzzleOne {
    fn new() -> DayOnePuzzleOne {
        DayOnePuzzleOne {
            list_one: vec![],
            list_two: vec![],
        }
    }
    fn parse(&mut self, s: &str) {
        let lines: Vec<&str> = s.split("\n").collect();
        lines.iter().for_each(|line| {
            let numbers: Vec<&str> = line.split("   ").collect();
            let x: usize = numbers
                .get(0)
                .expect("Could not parse the string properly")
                .parse()
                .expect("Could not convert to number");
            let y: usize = numbers
                .get(1)
                .expect("Could not parse the string properly")
                .parse()
                .expect("Could not convert to number");
            self.list_one.push(x);
            self.list_two.push(y)
        });
    }

    fn solve(&mut self) -> String {
        self.list_one.sort();
        self.list_two.sort();
        let mut distance = 0;
        for i in 0..self.list_one.len() {
            let x = self.list_one.get(i).expect("List length problems");
            let y = self.list_two.get(i).expect("List length problems");
            let d = x.abs_diff(*y);
            distance += d;
        }

        return distance.to_string();
    }
}

#[test]
fn test_parse() {
    let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";

    let mut day_one_solver = DayOnePuzzleOne::new();
    day_one_solver.parse(input);
    assert_eq!(
        day_one_solver,
        DayOnePuzzleOne {
            list_one: vec![3, 4, 2, 1, 3, 3],
            list_two: vec![4, 3, 5, 3, 9, 3]
        }
    )
}

#[test]
fn test_solve() {
    let mut day_one_solver = DayOnePuzzleOne {
        list_one: vec![3, 4, 2, 1, 3, 3],
        list_two: vec![4, 3, 5, 3, 9, 3],
    };
    let solution = day_one_solver.solve();
    assert_eq!(solution, "11");
}

#[derive(PartialEq, Debug, Clone)]
pub struct DayOnePuzzleTwo {
    list_one: Vec<usize>,
    list_two: Vec<usize>,
}

impl Solver for DayOnePuzzleTwo {
    fn new() -> DayOnePuzzleTwo {
        DayOnePuzzleTwo {
            list_one: vec![],
            list_two: vec![],
        }
    }
    fn parse(&mut self, s: &str) {
        let lines: Vec<&str> = s.split("\n").collect();
        lines.iter().for_each(|line| {
            let numbers: Vec<&str> = line.split("   ").collect();
            let x: usize = numbers
                .get(0)
                .expect("Could not parse the string properly")
                .parse()
                .expect("Could not convert to number");
            let y: usize = numbers
                .get(1)
                .expect("Could not parse the string properly")
                .parse()
                .expect("Could not convert to number");
            self.list_one.push(x);
            self.list_two.push(y)
        });
    }

    fn solve(&mut self) -> String {
        let mut similarity_score = 0;
        self.list_one.iter().for_each(|x| {
            let mut counter = 0;
            self.list_two.iter().for_each(|y| {
                if x == y {
                    counter += 1
                }
            });
            similarity_score += counter * x;
        });

        return similarity_score.to_string();
    }
}

#[test]
fn test_parse_two() {
    let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";

    let mut solver = DayOnePuzzleTwo::new();
    solver.parse(input);
    assert_eq!(
        solver,
        DayOnePuzzleTwo {
            list_one: vec![3, 4, 2, 1, 3, 3],
            list_two: vec![4, 3, 5, 3, 9, 3]
        }
    )
}

#[test]
fn test_solve_two() {
    let mut solver = DayOnePuzzleTwo {
        list_one: vec![3, 4, 2, 1, 3, 3],
        list_two: vec![4, 3, 5, 3, 9, 3],
    };
    let solution = solver.solve();
    assert_eq!(solution, "31");
}
