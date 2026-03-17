mod puzzle;

use puzzle::{
    generate_snail_goal, is_solvable, parse_file, search, select_heuristic, select_search_mode,
    SearchMode,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run <filename>");
        return;
    }

    match parse_file(&args[1]) {
        Ok(initial_state) => {
            let goal_board = generate_snail_goal(initial_state.size);

            println!("--- Puzzle Info ---");
            println!("N: {}", initial_state.size);
            println!("Initial: {:?}", initial_state.board);
            println!("Goal:    {:?}", goal_board);

            if is_solvable(initial_state.size, &initial_state.board, &goal_board) {
                println!("\x1b[32mResult: This puzzle is SOLVABLE.\x1b[0m");
            } else {
                println!("\x1b[31mResult: This puzzle is UNSOLVABLE.\x1b[0m");
                return;
            }

            let heuristic = select_heuristic();
            let mode: SearchMode = select_search_mode();

            println!("\nSearching for solution...\n");
            match search(initial_state, &goal_board, heuristic.as_ref(), mode) {
                Ok(metrics) => {
                    println!("\x1b[32m=== SOLUTION FOUND ===\x1b[0m");
                    println!("Total states opened: {}", metrics.total_states_opened);
                    println!("Max states in memory: {}", metrics.max_states_in_memory);
                    println!(
                        "Solution length: {} moves\n",
                        metrics.solution_path.len() - 1
                    );

                    println!("Solution path:");
                    for (i, state) in metrics.solution_path.iter().enumerate() {
                        println!("Step {}: {:?}", i, state.board);
                    }
                }
                Err(e) => {
                    println!("\x1b[31mError: {}\x1b[0m", e);
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
