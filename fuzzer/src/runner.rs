use crate::FuzzTarget;

pub mod process;

pub trait Runner {
    /// Runs the given Target
    fn run(&self, target: FuzzTarget) -> Option<Vec<Vec<u8>>>;
}
