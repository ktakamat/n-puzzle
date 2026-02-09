mod puzzle;

use puzzle::{parse_file, generate_snail_goal, is_solvable, AstarSolver, HeuristicType};
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run -- <filename>");
        return;
    }

    match parse_file(&args[1]) {
        Ok(initial_state) => {
            let goal_board = generate_snail_goal(initial_state.size);
            
            println!("--- Puzzle Info ---");
            println!("N: {}", initial_state.size);
            println!("Initial: {:?}", initial_state.board);
            println!("Goal:    {:?}", goal_board);
            
            if !is_solvable(initial_state.size, &initial_state.board, &goal_board) {
                println!("\x1b[31mResult: This puzzle is UNSOLVABLE.\x1b[0m");
                return;
            }

            println!("\x1b[32mResult: This puzzle is SOLVABLE.\x1b[0m\n");

            // Ask user to choose heuristic
            println!("Choose a heuristic:");
            println!("1. Manhattan Distance");
            println!("2. Linear Conflict Manhattan");
            println!("3. Euclidean Distance");
            print!("Enter choice (1-3): ");
            io::stdout().flush().unwrap();

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("Failed to read input");
            let choice: u8 = choice.trim().parse().unwrap_or(1);

            let heuristic_type = match choice {
                1 => HeuristicType::Manhattan,
                2 => HeuristicType::LinearConflictManhattan,
                3 => HeuristicType::Euclidean,
                _ => HeuristicType::Manhattan,
            };

            // Solve the puzzle
            match AstarSolver::new(initial_state, heuristic_type) {
                Ok(mut solver) => {
                    println!("Solving...\n");
                    match solver.solve() {
                        Some(stats) => {
                            println!("\n--- Solution Found ---");
                            println!("States evaluated: {}", stats.states_evaluated);
                            println!("Max states in memory: {}", stats.max_states_in_memory);
                            println!("Number of moves: {}", stats.num_moves);
                            println!("\nSolution path:");
                            for (i, state) in stats.solution_path.iter().enumerate() {
                                println!("Step {}: {:?}", i, state);
                            }
                        }
                        None => println!("No solution found!"),
                    }
                }
                Err(e) => println!("Error: {}", e),
            }
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}
