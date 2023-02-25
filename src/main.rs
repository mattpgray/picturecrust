use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Hash)]
enum CellState {
    Unknown,
    Dot,
    Cross,
}

type Setting = Vec<i32>;

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
        let row_boards = self.generate_row_boards();
        let col_boards = self.generate_col_boards();
        let row_sets: HashSet<Board> = HashSet::from_iter(row_boards);
        for col_board in &col_boards {
            if row_sets.contains(col_board) {
                return Some(col_board.clone())
            }
        }
        None
    }

    fn generate_row_boards(&self) -> Vec<Board> {
        let rows = self.row_settings.len();
        let cols = self.col_settings.len();

        let row_proposals = Vec::from_iter(
            self.row_settings
                .iter()
                .map(|settings| proposals(cols, settings)),
        );

        let mut boards: Vec<Board> = Vec::new();
        let empty_board = Board::new(rows, cols);
        self.prop_boards_rows(&mut boards, empty_board, &row_proposals, 0);
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

    fn generate_col_boards(&self) -> Vec<Board> {
        let rows = self.row_settings.len();
        let cols = self.col_settings.len();


        let col_proposals = Vec::from_iter(self
            .col_settings
            .iter()
            .map(|settings| proposals(rows, settings)));

        let mut boards: Vec<Board> = Vec::new();
        let empty_board = Board::new(rows, cols);
        self.prop_boards_cols(&mut boards, empty_board, &col_proposals, 0);
        boards
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

fn main() {
    let row_settings = vec![
        vec![9],
        vec![2, 1, 1, 1, 2],
        vec![1, 1, 1, 1, 1],
        vec![2, 1, 1, 1, 2],
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
        vec![],
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
