use std::collections::{HashMap, VecDeque};
use crate::puzzle::state::State;

/// An Additive Pattern Database (APD) with abstraction
/// Stores pre-computed optimal distances for disjoint tile patterns
pub struct AdditivePatternDatabase {
    databases: Vec<PatternDB>,
    board_size: usize,
}

struct PatternDB {
    pattern_tiles: Vec<u16>,
    costs: HashMap<Vec<u16>, u32>,
    goal_positions: HashMap<u16, usize>,
}

impl AdditivePatternDatabase {
    /// Create a new APD for the given board size and goal state
    pub fn new(board_size: usize, goal_board: &[u16]) -> Self {
        let pattern_tiles = Self::create_patterns(board_size);
        let mut databases = Vec::new();

        for pattern in pattern_tiles {
            let db = PatternDB::new(&pattern, goal_board, board_size);
            databases.push(db);
        }

        AdditivePatternDatabase {
            databases,
            board_size,
        }
    }

    /// Create disjoint tile groups for patterns
    /// Strategy: group tiles in chunks of 8-10 depending on board size
    fn create_patterns(board_size: usize) -> Vec<Vec<u16>> {
        let total_tiles = (board_size * board_size) as u16;
        let pattern_size = if board_size <= 4 {
            6  // Smaller patterns for small boards
        } else {
            8  // 8-tile patterns for larger boards
        };

        let mut patterns = Vec::new();
        let mut current_pattern = Vec::new();

        for tile in 1..=total_tiles {
            if tile == 0 {
                continue;
            }
            current_pattern.push(tile);

            if current_pattern.len() == pattern_size as usize {
                patterns.push(current_pattern.clone());
                current_pattern.clear();
            }
        }

        // Add remaining tiles to last pattern
        if !current_pattern.is_empty() {
            patterns.push(current_pattern);
        }

        patterns
    }

    /// Get heuristic estimate by summing all pattern database values
    pub fn estimate(&self, state: &State, goal_board: &[u16]) -> u32 {
        let mut total_cost = 0u32;

        for db in &self.databases {
            let pattern_state = db.extract_pattern(&state.board, self.board_size);
            if let Some(&cost) = db.costs.get(&pattern_state) {
                total_cost += cost;
            }
        }

        total_cost
    }
}

impl PatternDB {
    fn new(pattern_tiles: &[u16], goal_board: &[u16], board_size: usize) -> Self {
        let mut goal_positions = HashMap::new();

        for (pos, &tile) in goal_board.iter().enumerate() {
            if pattern_tiles.contains(&tile) {
                goal_positions.insert(tile, pos);
            }
        }

        let mut db = PatternDB {
            pattern_tiles: pattern_tiles.to_vec(),
            costs: HashMap::new(),
            goal_positions,
        };

        // Generate PDB using BFS from goal state
        db.generate_pdb(goal_board, board_size);

        db
    }

    /// Generate the pattern database using backwards BFS from goal state
    fn generate_pdb(&mut self, goal_board: &[u16], board_size: usize) {
        let mut queue = VecDeque::new();
        let mut visited = std::collections::HashSet::new();

        // Extract pattern from goal state
        let goal_pattern = self.extract_pattern(goal_board, board_size);
        queue.push_back((goal_pattern.clone(), 0u32));
        visited.insert(goal_pattern.clone());
        self.costs.insert(goal_pattern, 0);

        // BFS backwards from goal
        while let Some((current_pattern, cost)) = queue.pop_front() {
            // Generate neighbors by simulating moves
            let neighbors = self.generate_neighbor_patterns(&current_pattern, board_size);

            for neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor.clone());
                    self.costs.insert(neighbor.clone(), cost + 1);
                    queue.push_back((neighbor, cost + 1));
                }
            }
        }
    }

    /// Extract the pattern state from a full board state
    fn extract_pattern(&self, board: &[u16], board_size: usize) -> Vec<u16> {
        let mut pattern = Vec::new();

        for &tile in &self.pattern_tiles {
            let pos = board.iter().position(|&x| x == tile).unwrap_or(board_size * board_size);
            pattern.push(pos as u16);
        }

        pattern
    }

    /// Generate neighbor patterns by simulating legal moves on the pattern
    fn generate_neighbor_patterns(
        &self,
        pattern: &[u16],
        board_size: usize,
    ) -> Vec<Vec<u16>> {
        let mut neighbors = Vec::new();
        let size = board_size as i32;

        // For each tile in the pattern, try to move it
        for i in 0..self.pattern_tiles.len() {
            let pos = pattern[i] as usize;
            let row = (pos as i32) / size;
            let col = (pos as i32) % size;

            // Try moving the tile to each direction
            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let new_row = row + dr;
                let new_col = col + dc;

                if new_row >= 0 && new_row < size && new_col >= 0 && new_col < size {
                    let new_pos = ((new_row * size + new_col) as usize) as u16;
                    let mut new_pattern = pattern.to_vec();
                    new_pattern[i] = new_pos;
                    neighbors.push(new_pattern);
                }
            }
        }

        neighbors
    }
}
