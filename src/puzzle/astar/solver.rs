use std::collections::{BinaryHeap, HashMap, HashSet};
use crate::puzzle::state::State;
use crate::puzzle::goal::generate_snail_goal;
use crate::puzzle::validator::is_solvable;
use crate::puzzle::astar::heuristic::{
    Heuristic,
    ManhattanDistance,
    LinearConflictManhattan,
    EuclideanDistance,
};

use crate::puzzle::astar::node::Node;

pub enum HeuristicType {
    Manhattan,
    LinearConflictManhattan,
    Euclidean,
}

#[allow(dead_code)]
pub struct AstarSolver {
    initial_state: State,
    goal_state_board: Vec<u16>,
    heuristic: Box<dyn Heuristic>,
    size: usize,
}

impl AstarSolver {
    pub fn new(initial_state: State, heuristic_type: HeuristicType) -> Result<Self, String> {
        let size = initial_state.size;
        let goal_state_board = generate_snail_goal(size);

        if !is_solvable(size, &initial_state.board, &goal_state_board) {
            return Err("Puzzle is unsolvable!".to_string());
        }

        let heuristic: Box<dyn Heuristic> = match heuristic_type {
            HeuristicType::Manhattan => Box::new(ManhattanDistance::new(size)),
            HeuristicType::LinearConflictManhattan => Box::new(LinearConflictManhattan::new(size)),
            HeuristicType::Euclidean => Box::new(EuclideanDistance::new(size)),
        };

        Ok(AstarSolver {
            initial_state,
            goal_state_board,
            heuristic,
            size,
        })
    }

    pub fn solve(&mut self) -> Option<SolutionStats> {
        let mut open_set = BinaryHeap::new();
        let mut g_scores: HashMap<Vec<u16>, u32> = HashMap::new();
        let mut came_from: HashMap<Vec<u16>, Vec<u16>> = HashMap::new();
        let mut closed_set: HashSet<Vec<u16>> = HashSet::new();

        let initial_h_cost = self.heuristic.calculate(&self.initial_state, &self.goal_state_board);
        let initial_node = Node::new(self.initial_state.clone(), 0, initial_h_cost, None);

        open_set.push(initial_node.clone());
        g_scores.insert(self.initial_state.board.clone(), 0);

        let mut states_evaluated_count = 0;
        let mut max_states_in_memory = 1;

        while let Some(current_node) = open_set.pop() {
            states_evaluated_count += 1;

            if current_node.state.board == self.goal_state_board {
                return Some(self.reconstruct_path(came_from, current_node.state.board, states_evaluated_count, max_states_in_memory));
            }

            closed_set.insert(current_node.state.board.clone());

            for mut neighbor_state in current_node.state.get_neighbors() {
                if closed_set.contains(&neighbor_state.board) {
                    continue;
                }

                let tentative_g_score = current_node.g_cost + 1;

                let existing_g_score = *g_scores.get(&neighbor_state.board).unwrap_or(&u32::MAX);

                if tentative_g_score < existing_g_score {
                    neighbor_state.g = tentative_g_score;
                    neighbor_state.h = self.heuristic.calculate(&neighbor_state, &self.goal_state_board);
                    
                    let neighbor_node = Node::new(
                        neighbor_state.clone(),
                        tentative_g_score,
                        neighbor_state.h,
                        Some(current_node.state.board.clone()),
                    );

                    g_scores.insert(neighbor_state.board.clone(), tentative_g_score);
                    came_from.insert(neighbor_state.board.clone(), current_node.state.board.clone());
                    open_set.push(neighbor_node);

                    max_states_in_memory = max_states_in_memory.max(open_set.len() + closed_set.len());
                }
            }   
            println!("evaluated {} states!", states_evaluated_count)
        }

        None // No solution found
    }

    fn reconstruct_path(
        &self,
        came_from: HashMap<Vec<u16>, Vec<u16>>,
        mut current_board: Vec<u16>,
        states_evaluated_count: usize,
        max_states_in_memory: usize,
    ) -> SolutionStats {
        let mut total_path = Vec::new();
        let mut num_moves = 0;

        while let Some(prev_board) = came_from.get(&current_board) {
            total_path.push(current_board.clone());
            current_board = prev_board.clone();
            num_moves += 1;
        }
        total_path.push(current_board.clone()); // Add the initial state
        total_path.reverse(); // Reverse to get path from start to goal

        SolutionStats {
            states_evaluated: states_evaluated_count,
            max_states_in_memory,
            num_moves,
            solution_path: total_path,
        }
    }
}

pub struct SolutionStats {
    pub states_evaluated: usize,
    pub max_states_in_memory: usize,
    pub num_moves: u32,
    pub solution_path: Vec<Vec<u16>>,
}
