fn get_inversion_count(board: &[u16]) -> usize {
    let mut inv_count = 0;
    let len = board.len();
    for i in 0..len {
        for j in i + 1..len {
            if board[i] != 0 && board[j] != 0 && board[i] > board[j] {
                inv_count += 1;
            }
        }
    }
    inv_count
}

fn get_blank_row(board: &[u16], size: usize) -> usize {
    let pos = board.iter().position(|&x| x == 0).unwrap_or(0);
    pos / size
}

pub fn is_solvable(size: usize, initial_board: &[u16], goal_board: &[u16]) -> bool {
    let inv_initial = get_inversion_count(initial_board);
    let inv_goal = get_inversion_count(goal_board);

    if size % 2 != 0 {
        inv_initial % 2 == inv_goal % 2
    } else {
        let row_initial = get_blank_row(initial_board, size);
        let row_goal = get_blank_row(goal_board, size);
        
        (inv_initial % 2 == inv_goal % 2) == (row_initial % 2 == row_goal % 2)
    }
}