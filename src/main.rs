use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Hash)]
enum CellState {
    Unknown,
    Dot,
    Cross,
}

type Setting = Vec<i32>;

fn all_proposals(len: usize, all_settings: &Vec<Setting>) -> Vec<Vec<Proposal>> {
    Vec::from_iter(all_settings.iter().map(|settings| proposals(len, settings)))
}

fn proposals(len: usize, settings: &Setting) -> Vec<Proposal> {
    gen_proposal(len, settings)
}

fn gen_proposal(len: usize, settings: &[i32]) -> Vec<Proposal> {
    if settings.len() == 0 {
        return vec![];
    }

    let n_dots: i32 = settings.iter().sum();
    let max_space: usize = n_dots as usize + settings.len() - 1;
    let remaining_space = len - max_space;

    let mut proposals: Vec<Proposal> = Vec::new();
    for i in 0..remaining_space + 1 {
        let mut head: Vec<CellState> = Vec::new();
        for _ in 0..i {
            head.push(CellState::Cross);
        }
        for _ in 0..settings[0] {
            head.push(CellState::Dot);
        }

        if settings.len() > 1 {
            head.push(CellState::Cross);
            let space_taken = i + settings[0] as usize + 1;
            let tails = gen_proposal(len - space_taken, &settings[1..]);
            for tail in tails {
                let proposal = [head.clone(), tail].concat();
                proposals.push(proposal);
            }
        } else {
            let space_taken = i + settings[0] as usize;
            for _ in 0..len - space_taken {
                head.push(CellState::Cross);
            }
            proposals.push(head);
        }
    }

    proposals
}

struct BoardSolver {
    row_settings: Vec<Setting>,
    col_settings: Vec<Setting>,
}

type Proposal = Vec<CellState>;

impl BoardSolver {
    fn from_settings(row_settings: Vec<Setting>, col_settings: Vec<Setting>) -> Self {
        Self {
            row_settings,
            col_settings,
        }
    }

    fn solve(&self) -> Option<Board> {
        let rows = self.row_settings.len();
        let cols = self.col_settings.len();
        // All possible proposals
        let mut row_proposals = all_proposals(cols, &self.row_settings);
        let mut col_proposals = all_proposals(rows, &self.col_settings);

        // Filter down the list of possible proposals until we reach the limit.
        let mut n_row_boards = n_boards(&row_proposals);
        let mut n_col_boards = n_boards(&row_proposals);
        loop {
            (row_proposals, col_proposals) = reduce_proposals(row_proposals, col_proposals);
            let next_row_boards = n_boards(&row_proposals);
            let next_col_boards = n_boards(&col_proposals);

            // Invalid board settings.
            if next_row_boards == 0 || next_col_boards == 0 {
                return None;
            }

            // The base case. We are done.
            if next_row_boards == 1 && next_col_boards == 1 {
                break;
            }
            // If we cannot reduce further we have to leave the loop
            if next_row_boards == n_row_boards && next_col_boards == n_col_boards {
                break;
            }

            n_row_boards = next_row_boards;
            n_col_boards = next_col_boards;
        }

        // We can now use brute force to find the winning solution.
        // 1. Generate all possible boards given the row proposals.
        // 2. Generate all possible boards given the col proposals.
        // 3. Find the board that is in both of these sets of boards.
        //    That is the winning board.
        let row_boards = self.generate_row_boards(rows, cols, &row_proposals);
        let col_boards = self.generate_col_boards(rows, cols, &col_proposals);
        let row_sets: HashSet<Board> = HashSet::from_iter(row_boards);
        for col_board in &col_boards {
            if row_sets.contains(col_board) {
                return Some(col_board.clone());
            }
        }
        None
    }

    fn generate_row_boards(
        &self,
        rows: usize,
        cols: usize,
        row_proposals: &Vec<Vec<Proposal>>,
    ) -> Vec<Board> {
        let mut boards: Vec<Board> = Vec::new();
        let empty_board = Board::new(rows, cols);
        self.prop_boards_rows(&mut boards, empty_board, row_proposals, 0);
        boards
    }

    fn generate_col_boards(
        &self,
        rows: usize,
        cols: usize,
        col_proposals: &Vec<Vec<Proposal>>,
    ) -> Vec<Board> {
        let mut boards: Vec<Board> = Vec::new();
        let empty_board = Board::new(rows, cols);
        self.prop_boards_cols(&mut boards, empty_board, col_proposals, 0);
        boards
    }

    fn prop_boards_rows(
        &self,
        boards: &mut Vec<Board>,
        current_board: Board,
        props: &[Vec<Proposal>],
        row: usize,
    ) {
        if props.len() == 0 {
            boards.push(current_board);
            return;
        }

        for prop in &props[0] {
            let mut next_board = current_board.clone();
            for (col, cell) in prop.iter().enumerate() {
                next_board.set_cell(row, col, cell.to_owned());
            }

            self.prop_boards_rows(boards, next_board, &props[1..], row + 1);
        }
    }

    fn prop_boards_cols(
        &self,
        boards: &mut Vec<Board>,
        current_board: Board,
        props: &[Vec<Proposal>],
        col: usize,
    ) {
        if props.len() == 0 {
            boards.push(current_board);
            return;
        }

        for prop in &props[0] {
            let mut next_board = current_board.clone();
            for (row, cell) in prop.iter().enumerate() {
                next_board.set_cell(row, col, cell.to_owned());
            }

            self.prop_boards_cols(boards, next_board, &props[1..], col + 1);
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Board {
    cells: Vec<CellState>,
    rows: usize,
    cols: usize,
}

impl Board {
    fn new(rows: usize, cols: usize) -> Self {
        let cells = vec![CellState::Unknown; rows * cols];
        Self { rows, cols, cells }
    }

    fn get_cell(&self, row: usize, col: usize) -> &CellState {
        &self.cells[row * self.cols + col]
    }

    fn set_cell(&mut self, row: usize, col: usize, cell: CellState) {
        self.cells[row * self.cols + col] = cell;
    }

    fn display(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let cell = self.get_cell(row, col);
                match cell {
                    CellState::Unknown => print!("?"),
                    CellState::Dot => print!("█"),
                    CellState::Cross => print!("X"),
                }
            }
            println!();
        }
    }
}

fn n_boards(all_proposals: &Vec<Vec<Proposal>>) -> usize {
    let mut tot: usize = 1;
    for props in all_proposals {
        tot *= props.len();
    }
    tot
}

fn reduce_board(
    board_proposals: &Vec<Vec<Proposal>>,
    check_proposals: &Vec<Vec<Proposal>>,
    transpose: bool,
) -> Vec<Vec<Proposal>> {
    let mut board = if transpose {
        Board::new(check_proposals.len(), board_proposals.len())
    } else {
        Board::new(board_proposals.len(), check_proposals.len())
    };

    // Generate a board that only contains cell states that we know for certain
    // as all of the proposals for this row/col match.
    for (i, props) in board_proposals.iter().enumerate() {
        for (prop_idx, prop) in props.iter().enumerate() {
            if prop_idx == 0 {
                for (j, cell) in prop.iter().enumerate() {
                    if transpose {
                        board.set_cell(j, i, cell.clone());
                    } else {
                        board.set_cell(i, j, cell.clone());
                    }
                }
                continue;
            }
            for (j, cell) in prop.iter().enumerate() {
                let board_cell = if transpose {
                    board.get_cell(j, i)
                } else {
                    board.get_cell(i, j)
                };

                if board_cell.to_owned() == CellState::Unknown {
                    continue;
                }

                if cell.to_owned() != board_cell.to_owned() {
                    if transpose {
                        board.set_cell(j, i, CellState::Unknown);
                    } else {
                        board.set_cell(i, j, CellState::Unknown);
                    }
                }
            }
        }
    }

    // Now we validate against the set of settings to be able to reduce how many options they have.
    let mut valid_props: Vec<Vec<Proposal>> = Vec::new();
    for (j, prop) in check_proposals.iter().enumerate() {
        let valid_prop = prop.iter().cloned().filter(|prop| {
            for (i, cell) in prop.iter().enumerate() {
                let board_cell = if transpose {
                    board.get_cell(j, i)
                } else {
                    board.get_cell(i, j)
                };

                if board_cell.to_owned() == CellState::Unknown {
                    continue;
                }

                if cell.to_owned() != board_cell.to_owned() {
                    return false;
                }
            }
            true
        });
        valid_props.push(Vec::from_iter(valid_prop));
    }

    valid_props
}

fn reduce_proposals(
    row_proposals: Vec<Vec<Proposal>>,
    col_proposals: Vec<Vec<Proposal>>,
) -> (Vec<Vec<Proposal>>, Vec<Vec<Proposal>>) {
    let col_proposals = reduce_board(&row_proposals, &col_proposals, false);
    let row_proposals = reduce_board(&col_proposals, &row_proposals, true);
    (row_proposals, col_proposals)
}

fn main() {
    let row_settings = vec![
        vec![9],
        vec![2, 1, 1, 1, 2],
        vec![1, 1, 1, 1, 1],
        vec![2, 1, 1, 1, 2],
        vec![2, 2, 1, 2, 2],
        vec![13],
        vec![1, 1],
        vec![15],
        vec![15],
        vec![3, 3, 3],
        vec![4, 5, 4],
        vec![4, 2, 2, 4],
        vec![6, 6],
        vec![15],
        vec![13],
    ];
    let col_settings = vec![
        vec![8],
        vec![3, 8],
        vec![5, 8],
        vec![2, 1, 2, 5],
        vec![1, 2, 2, 3],
        vec![6, 2, 5],
        vec![1, 1, 5, 2],
        vec![6, 4, 2],
        vec![1, 1, 5, 2],
        vec![6, 2, 5],
        vec![1, 2, 2, 3],
        vec![2, 1, 2, 5],
        vec![5, 8],
        vec![3, 8],
        vec![8],
    ];
    let board_solver = BoardSolver::from_settings(row_settings, col_settings);
    let board = board_solver.solve();
    match board {
        Some(board) => board.display(),
        None => eprintln!("Could not solve board"),
    }
}
