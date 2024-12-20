use crate::helpers::Solver;

#[derive(Clone, PartialEq, Debug)]
pub struct DayTwoPuzzleOne {
    reports: Vec<Vec<usize>>,
}

impl Solver for DayTwoPuzzleOne {
    fn new() -> DayTwoPuzzleOne
    where
        Self: Sized,
    {
        DayTwoPuzzleOne { reports: vec![] }
    }

    fn parse(&mut self, string: &str) -> () {
        let lines: Vec<&str> = string.split("\n").collect();
        lines.iter().for_each(|line| {
            let numbers_as_string: Vec<&str> = line.split(" ").collect();
            let numbers: Vec<usize> = numbers_as_string
                .iter()
                .map(|s| return s.parse().unwrap())
                .collect();
            self.reports.push(numbers)
        });
    }

    fn solve(&mut self) -> String {
        let mut safe_reports_counter = 0;
        self.reports.iter().for_each(|report| {
            let is_safe = report_is_safe(report);
            if is_safe {
                safe_reports_counter += 1;
            }
        });
        return safe_reports_counter.to_string();
    }
}

#[test]
fn test_parse() {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    let expected_output: Vec<Vec<usize>> = vec![
        vec![7, 6, 4, 2, 1],
        vec![1, 2, 7, 8, 9],
        vec![9, 7, 6, 2, 1],
        vec![1, 3, 2, 4, 5],
        vec![8, 6, 4, 4, 1],
        vec![1, 3, 6, 7, 9],
    ];
    let mut solver = DayTwoPuzzleOne::new();
    solver.parse(input);
    assert_eq!(
        solver,
        DayTwoPuzzleOne {
            reports: expected_output
        }
    )
}

#[test]
fn test_solve() {
    let mut solver = DayTwoPuzzleOne {
        reports: vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ],
    };

    assert_eq!(solver.solve(), "2");
}

#[test]
fn test_solve_2() {
    let mut solver = DayTwoPuzzleOne {
        reports: vec![vec![47, 45, 42, 41, 38, 37, 34, 32]],
    };

    assert_eq!(solver.solve(), "1");
}

#[derive(Clone, PartialEq, Debug)]
pub struct DayTwoPuzzleTwo {
    reports: Vec<Vec<usize>>,
}

impl Solver for DayTwoPuzzleTwo {
    fn new() -> DayTwoPuzzleTwo
    where
        Self: Sized,
    {
        DayTwoPuzzleTwo { reports: vec![] }
    }

    fn parse(&mut self, string: &str) -> () {
        let lines: Vec<&str> = string.split("\n").collect();
        lines.iter().for_each(|line| {
            let numbers_as_string: Vec<&str> = line.split(" ").collect();
            let numbers: Vec<usize> = numbers_as_string
                .iter()
                .map(|s| return s.parse().unwrap())
                .collect();
            self.reports.push(numbers)
        });
    }

    fn solve(&mut self) -> String {
        let mut safe_reports_counter = 0;
        self.reports.iter().for_each(|report| {
            let is_safe = report_is_safe(report);
            if is_safe {
                safe_reports_counter += 1;
            } else {
                for i in 0..report.len() {
                    let mut cloned_report = report.clone();
                    cloned_report.remove(i);
                    if report_is_safe(&cloned_report) {
                        safe_reports_counter += 1;
                        break;
                    }
                }
            }
        });
        return safe_reports_counter.to_string();
    }
}

fn report_is_safe(report: &Vec<usize>) -> bool {
    let mut is_safe = true;
    for i in 2..report.len() {
        let x = report.get(i - 2).unwrap();
        let y = report.get(i - 1).unwrap();
        let z = report.get(i).unwrap();
        if x == y || x == z || y == z {
            is_safe = false;
        }
        if z > y && y < x {
            is_safe = false;
        }
        if z < y && y > x {
            is_safe = false
        }
        if z.abs_diff(*y) > 3 || y.abs_diff(*x) > 3 {
            is_safe = false
        }
    }
    return is_safe;
}

#[test]
fn test_solve_puzzle_two() {
    let mut solver = DayTwoPuzzleTwo {
        reports: vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ],
    };

    assert_eq!(solver.solve(), "4");
}
