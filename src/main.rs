mod puzzle;

use puzzle::{parse_file, generate_snail_goal, is_solvable};

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
            
            if is_solvable(initial_state.size, &initial_state.board, &goal_board) {
                println!("\x1b[32mResult: This puzzle is SOLVABLE.\x1b[0m");
            } else {
                println!("\x1b[31mResult: This puzzle is UNSOLVABLE.\x1b[0m");
                return;
            }

            let neighbors = initial_state.get_neighbors();
            println!("Possible moves: {}", neighbors.len());
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}