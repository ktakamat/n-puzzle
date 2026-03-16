use crate::puzzle::heuristic::Heuristic;
use crate::puzzle::state::State;
use std::collections::HashSet;

pub struct SearchMetrics {
    pub total_states_opened: usize,
    pub max_states_in_memory: usize,
    pub solution_path: Vec<State>,
}

// start of IDA*; Manages the search and stats; g starts at 0
pub fn search(
    initial_state: State,
    goal_board: &[u16],
    heuristic: &dyn Heuristic,
) -> Result<SearchMetrics, String> {
    let goal_h = heuristic.estimate(&initial_state, goal_board);
    let mut threshold = goal_h;
    let mut total_states_opened;
    let mut max_states_in_memory = 0;

    loop {
        let mut visited_this_iteration = HashSet::new();
        let mut iteration_stats = IterationStats { states_opened: 0 };

        let (found, path, next_threshold) = search_recursive(
            &initial_state,
            goal_board,
            heuristic,
            0,
            threshold,
            &mut visited_this_iteration,
            &mut iteration_stats,
        );

        total_states_opened = iteration_stats.states_opened;
        max_states_in_memory = max_states_in_memory.max(visited_this_iteration.len());

        if found {
            return Ok(SearchMetrics {
                total_states_opened,
                max_states_in_memory,
                solution_path: path,
            });
        }

        if next_threshold == u32::MAX {
            return Err("No solution found".to_string());
        }

        threshold = next_threshold;
    }
}

struct IterationStats {
    states_opened: usize,
}

fn search_recursive(
    state: &State,
    goal_board: &[u16],
    heuristic: &dyn Heuristic,
    g: u32,
    threshold: u32,
    path_visited: &mut HashSet<Vec<u16>>,
    stats: &mut IterationStats,
) -> (bool, Vec<State>, u32) {
    let h = heuristic.estimate(state, goal_board);
    let f = g + h;

    stats.states_opened += 1;

    if f > threshold {
        return (false, Vec::new(), f);
    }

    if state.board == goal_board {
        return (true, vec![state.clone()], f);
    }

    if !path_visited.insert(state.board.clone()) {
        return (false, Vec::new(), u32::MAX);
    }

    let neighbors = state.get_neighbors();
    let mut min_exceeded_f = u32::MAX;

    for neighbor in neighbors {
        let (found, mut child_path, child_min_f) = search_recursive(
            &neighbor,
            goal_board,
            heuristic,
            g + 1,
            threshold,
            path_visited,
            stats,
        );

        if found {
            let mut path = vec![state.clone()];
            path.append(&mut child_path);
            return (true, path, f);
        }

        min_exceeded_f = min_exceeded_f.min(child_min_f);
    }

    path_visited.remove(&state.board);

    (false, Vec::new(), min_exceeded_f)
}
