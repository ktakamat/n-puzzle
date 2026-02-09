use crate::puzzle::state::State;
use crate::puzzle::goal::generate_snail_goal;

pub trait Heuristic {
    fn calculate(&self, current_state: &State, goal_state_board: &[u16]) -> u32;
}

pub struct ManhattanDistance {
    goal_positions: Vec<(usize, usize)>,
}

impl ManhattanDistance {
    pub fn new(size: usize) -> Self {
        let goal_board = generate_snail_goal(size);
        let mut goal_positions = vec![(0, 0); size * size];
        for (i, &tile) in goal_board.iter().enumerate() {
            goal_positions[tile as usize] = (i / size, i % size);
        }
        ManhattanDistance { goal_positions }
    }
}

impl Heuristic for ManhattanDistance {
    fn calculate(&self, current_state: &State, _goal_state_board: &[u16]) -> u32 {
        let mut distance = 0;
        let size = current_state.size;

        for (i, &tile) in current_state.board.iter().enumerate() {
            if tile == 0 {
                continue;
            }
            let (current_row, current_col) = (i / size, i % size);
            let (goal_row, goal_col) = self.goal_positions[tile as usize];
            distance += ((current_row as isize - goal_row as isize).abs() + (current_col as isize - goal_col as isize).abs()) as u32;
        }
        distance
    }
}

pub struct LinearConflictManhattan {
    manhattan: ManhattanDistance,
    goal_positions: Vec<(usize, usize)>,
}

impl LinearConflictManhattan {
    pub fn new(size: usize) -> Self {
        let goal_board = generate_snail_goal(size);
        let mut goal_positions = vec![(0, 0); size * size];
        for (i, &tile) in goal_board.iter().enumerate() {
            goal_positions[tile as usize] = (i / size, i % size);
        }
        LinearConflictManhattan {
            manhattan: ManhattanDistance::new(size),
            goal_positions,
        }
    }

    fn count_linear_conflicts(&self, current_board: &[u16], size: usize) -> u32 {
        let mut conflicts = 0;

        // Row conflicts
        for r in 0..size {
            for c1 in 0..size {
                let tile1 = current_board[r * size + c1];
                if tile1 == 0 {
                    continue;
                }
                let (goal_row1, goal_col1) = self.goal_positions[tile1 as usize];
                if goal_row1 != r {
                    continue;
                }
                for c2 in (c1 + 1)..size {
                    let tile2 = current_board[r * size + c2];
                    if tile2 == 0 {
                        continue;
                    }
                    let (goal_row2, goal_col2) = self.goal_positions[tile2 as usize];
                    if goal_row2 == r && goal_col1 > goal_col2 {
                        conflicts += 1;
                    }
                }
            }
        }

        // Column conflicts
        for c in 0..size {
            for r1 in 0..size {
                let tile1 = current_board[r1 * size + c];
                if tile1 == 0 {
                    continue;
                }
                let (goal_row1, goal_col1) = self.goal_positions[tile1 as usize];
                if goal_col1 != c {
                    continue;
                }
                for r2 in (r1 + 1)..size {
                    let tile2 = current_board[r2 * size + c];
                    if tile2 == 0 {
                        continue;
                    }
                    let (goal_row2, goal_col2) = self.goal_positions[tile2 as usize];
                    if goal_col2 == c && goal_row1 > goal_row2 {
                        conflicts += 1;
                    }
                }
            }
        }
        conflicts
    }
}

impl Heuristic for LinearConflictManhattan {
    fn calculate(&self, current_state: &State, goal_state_board: &[u16]) -> u32 {
        let manhattan_dist = self.manhattan.calculate(current_state, goal_state_board);
        let linear_conflicts = self.count_linear_conflicts(&current_state.board, current_state.size);
        manhattan_dist + (2 * linear_conflicts)
    }
}

pub struct EuclideanDistance {
    goal_positions: Vec<(usize, usize)>,
}

impl EuclideanDistance {
    pub fn new(size: usize) -> Self {
        let goal_board = generate_snail_goal(size);
        let mut goal_positions = vec![(0, 0); size * size];
        for (i, &tile) in goal_board.iter().enumerate() {
            goal_positions[tile as usize] = (i / size, i % size);
        }
        EuclideanDistance { goal_positions }
    }
}

impl Heuristic for EuclideanDistance {
    fn calculate(&self, current_state: &State, _goal_state_board: &[u16]) -> u32 {
        let mut distance = 0.0;
        let size = current_state.size;

        for (i, &tile) in current_state.board.iter().enumerate() {
            if tile == 0 {
                continue;
            }
            let (current_row, current_col) = (i / size, i % size);
            let (goal_row, goal_col) = self.goal_positions[tile as usize];

            let dr = current_row as f64 - goal_row as f64;
            let dc = current_col as f64 - goal_col as f64;
            distance += (dr * dr + dc * dc).sqrt();
        }
        distance.round() as u32
    }
}
