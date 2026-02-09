use crate::puzzle::state::State;
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Node {
    pub state: State,
    pub f_cost: u32,
    pub g_cost: u32,
    pub h_cost: u32,
    pub parent: Option<Vec<u16>>,
}

impl Node {
    pub fn new(state: State, g_cost: u32, h_cost: u32, parent_board: Option<Vec<u16>>) -> Self {
        Node {
            f_cost: g_cost + h_cost,
            g_cost,
            h_cost,
            state,
            parent: parent_board,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // A* needs a min-heap, so we invert the order for f_cost.
        // If f_cost is equal, prioritize higher g_cost (fewer moves to current state)
        // to break ties and potentially find shorter paths earlier (though not strictly necessary for correctness)
        other.f_cost.cmp(&self.f_cost)
            .then_with(|| self.g_cost.cmp(&other.g_cost))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
