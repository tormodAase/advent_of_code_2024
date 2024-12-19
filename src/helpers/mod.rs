use dyn_clone::DynClone;

pub trait Solver: DynClone {
    fn new() -> Self
    where
        Self: Sized;
    fn parse(&mut self, string: &str) -> ();

    fn solve(&mut self) -> String;
}

dyn_clone::clone_trait_object!(Solver);

/**
 * A collection contains all solvers for a given day
 */
#[derive(Clone)]
pub struct Collection {
    pub puzzle_1: Box<dyn Solver>,
    pub puzzle_2: Box<dyn Solver>,
}
