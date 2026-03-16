#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub board: Vec<u16>,
    pub size: usize,
    pub blank_pos: usize,
}

impl State {
    pub fn new(board: Vec<u16>, size: usize) -> Self {
        let blank_pos = board
            .iter()
            .position(|&x| x == 0)
            .expect("No blank (0) tile found");
        Self {
            board,
            size,
            blank_pos,
        }
    }

    // Generate new boards for neighbors
    pub fn get_neighbors(&self) -> Vec<State> {
        let mut neighbors = Vec::new();
        let size = self.size as isize;
        let r = (self.blank_pos as isize) / size;
        let c = (self.blank_pos as isize) % size;

        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nr = r + dr;
            let nc = c + dc;

            if nr >= 0 && nr < size && nc >= 0 && nc < size {
                let new_blank_pos = (nr * size + nc) as usize;
                let mut new_board = self.board.clone();
                new_board.swap(self.blank_pos, new_blank_pos);

                neighbors.push(State {
                    board: new_board,
                    size: self.size,
                    blank_pos: new_blank_pos,
                });
            }
        }
        neighbors
    }
}
