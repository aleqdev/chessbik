use chessbik_board::Board;
use chessbik_commons::Cell;

pub fn compute(board1: &Board<Cell>, board2: &Board<Cell>) -> Vec<usize> {
    let mut v = vec![];

    for (i, (c1, c2)) in board1.cells.iter().zip(board2.cells.iter()).enumerate() {
        if c1 != c2 {
            v.push(i);
        }
    }

    v
}

pub fn all() -> Vec<usize> {
    (0..54).collect()
}

pub fn compute_optional(board1: Option<&Board<Cell>>, board2: &Board<Cell>) -> Vec<usize> {
    match board1 {
        Some(board) => compute(board, board2),
        None => all(),
    }
}
