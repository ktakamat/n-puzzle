pub mod state;
pub mod goal;
pub mod parser;
pub mod validator;
pub mod astar;

pub use goal::generate_snail_goal;
pub use parser::parse_file;
pub use validator::is_solvable;

pub use astar::solver::{AstarSolver, HeuristicType, /*SolutionStats*/};
//pub use astar::heuristic::{Heuristic, ManhattanDistance, LinearConflictManhattan, EuclideanDistance};
//pub use astar::node::Node;
