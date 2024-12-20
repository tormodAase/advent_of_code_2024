use crate::day_one::{DayOnePuzzleOne, DayOnePuzzleTwo};
use crate::day_two::{DayTwoPuzzleOne, DayTwoPuzzleTwo};
use crate::helpers::*;

pub fn get_solvers() -> Vec<Collection> {
    let mut solvers = vec![
        Collection {
            puzzle_1: Box::new(DayOnePuzzleOne::new()),
            puzzle_2: Box::new(DayOnePuzzleTwo::new()),
        },
        Collection {
            puzzle_1: Box::new(DayTwoPuzzleOne::new()),
            puzzle_2: Box::new(DayTwoPuzzleTwo::new()),
        },
    ];

    for _ in 0..(25 - solvers.len()) {
        solvers.push(Collection {
            puzzle_1: Box::new(NotImplementedYet::new()),
            puzzle_2: Box::new(NotImplementedYet::new()),
        })
    }

    return solvers;
}

pub fn get_solver(
    solvers: &mut Vec<Collection>,
    day: usize,
    puzzle: usize,
) -> Option<Box<dyn Solver>> {
    if day == 0 {
        return None;
    }
    let collection = solvers.get(day - 1)?;
    if puzzle == 1 {
        return Some(collection.puzzle_1.to_owned());
    };
    if puzzle == 2 {
        return Some(collection.puzzle_2.to_owned());
    }
    return None;
}

#[derive(Clone)]
struct NotImplementedYet {}
impl Solver for NotImplementedYet {
    fn new() -> NotImplementedYet {
        NotImplementedYet {}
    }

    fn parse(&mut self, _: &str) -> () {}

    fn solve(&mut self) -> String {
        return String::from("Not implemented yet");
    }
}
