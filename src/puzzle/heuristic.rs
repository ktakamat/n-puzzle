use crate::puzzle::state::State;

pub trait Heuristic {
    fn estimate(&self, state: &State, goal: &[u16]) -> u32;
}

pub struct ManhattanDistance;
pub struct LinearConflictManhattan;
pub struct MisplacedTiles;

impl Heuristic for ManhattanDistance {
    fn estimate(&self, state: &State, goal: &[u16]) -> u32 {
        let mut distance = 0u32;
        let size = state.size as i32;

        for (i, &tile) in state.board.iter().enumerate() {
            if tile == 0 {
                continue;
            }

            let current_row = (i as i32) / size;
            let current_col = (i as i32) % size;

            if let Some(goal_pos) = goal.iter().position(|&x| x == tile) {
                let goal_row = (goal_pos as i32) / size;
                let goal_col = (goal_pos as i32) % size;

                distance +=
                    ((current_row - goal_row).abs() + (current_col - goal_col).abs()) as u32;
            }
        }

        distance
    }
}

impl Heuristic for LinearConflictManhattan {
    fn estimate(&self, state: &State, goal: &[u16]) -> u32 {
        let size = state.size as i32;
        let mut manhattan = 0u32;
        let mut conflicts = 0u32;

        for (i, &tile) in state.board.iter().enumerate() {
            if tile == 0 {
                continue;
            }

            let current_row = (i as i32) / size;
            let current_col = (i as i32) % size;

            if let Some(goal_pos) = goal.iter().position(|&x| x == tile) {
                let goal_row = (goal_pos as i32) / size;
                let goal_col = (goal_pos as i32) % size;

                manhattan +=
                    ((current_row - goal_row).abs() + (current_col - goal_col).abs()) as u32;
            }
        }

        for row in 0..size {
            for i in 0..size {
                let pos1 = (row * size + i) as usize;
                let tile1 = state.board[pos1];
                if tile1 == 0 {
                    continue;
                }

                if let Some(goal_pos1) = goal.iter().position(|&x| x == tile1) {
                    let goal_row1 = (goal_pos1 as i32) / size;
                    if goal_row1 != row {
                        continue;
                    }

                    for j in (i + 1)..size {
                        let pos2 = (row * size + j) as usize;
                        let tile2 = state.board[pos2];
                        if tile2 == 0 {
                            continue;
                        }

                        if let Some(goal_pos2) = goal.iter().position(|&x| x == tile2) {
                            let goal_row2 = (goal_pos2 as i32) / size;
                            if goal_row2 != row {
                                continue;
                            }

                            let goal_col1 = (goal_pos1 as i32) % size;
                            let goal_col2 = (goal_pos2 as i32) % size;

                            if goal_col1 > goal_col2 {
                                conflicts += 2;
                            }
                        }
                    }
                }
            }
        }

        for col in 0..size {
            for i in 0..size {
                let pos1 = (i * size + col) as usize;
                let tile1 = state.board[pos1];
                if tile1 == 0 {
                    continue;
                }

                if let Some(goal_pos1) = goal.iter().position(|&x| x == tile1) {
                    let goal_col1 = (goal_pos1 as i32) % size;
                    if goal_col1 != col {
                        continue;
                    }

                    for j in (i + 1)..size {
                        let pos2 = (j * size + col) as usize;
                        let tile2 = state.board[pos2];
                        if tile2 == 0 {
                            continue;
                        }

                        if let Some(goal_pos2) = goal.iter().position(|&x| x == tile2) {
                            let goal_col2 = (goal_pos2 as i32) % size;
                            if goal_col2 != col {
                                continue;
                            }

                            let goal_row1 = (goal_pos1 as i32) / size;
                            let goal_row2 = (goal_pos2 as i32) / size;

                            if goal_row1 > goal_row2 {
                                conflicts += 2;
                            }
                        }
                    }
                }
            }
        }

        manhattan + conflicts
    }
}

impl Heuristic for MisplacedTiles {
    fn estimate(&self, state: &State, goal: &[u16]) -> u32 {
        let mut count = 0u32;
        for (i, &tile) in state.board.iter().enumerate() {
            if tile != 0 && goal[i] != tile {
                count += 1;
            }
        }
        count
    }
}

pub fn select_heuristic() -> Box<dyn Heuristic> {
    println!("\n--- Select Heuristic ---");
    println!("1. Manhattan Distance");
    println!("2. Linear Conflict Manhattan");
    println!("3. Misplaced Tiles");
    print!("Choice (1-3): ");

    use std::io::{self, Write};
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim() {
        "1" => Box::new(ManhattanDistance),
        "2" => Box::new(LinearConflictManhattan),
        "3" => Box::new(MisplacedTiles),
        _ => {
            println!("Invalid choice, defaulting to Manhattan Distance");
            Box::new(ManhattanDistance)
        }
    }
}
