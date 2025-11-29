pub mod solver;

fn main() {
    let row_clues: Vec<Vec<usize>> = vec![
        vec![6, 1, 2],
        vec![5, 1, 3],
        vec![8, 5],
        vec![1, 3, 1, 5],
        vec![9],
        //
        vec![1, 1, 6],
        vec![1, 1, 2, 1],
        vec![1, 3, 1, 1],
        vec![2, 1],
        vec![6, 2],
        //
        vec![2, 7],
        vec![2, 5],
        vec![6, 1],
        vec![3, 1, 1],
        vec![3, 1, 1],
    ];

    let col_clues: Vec<Vec<usize>> = vec![
        vec![4, 3],
        vec![3, 3],
        vec![4, 1],
        vec![4, 3, 1],
        vec![5, 1, 2],
        //
        vec![1, 1, 4, 3],
        vec![1, 1, 1, 1, 2],
        vec![4, 1, 1, 1],
        vec![1, 3],
        vec![6, 3],
        //
        vec![6, 3],
        vec![5, 6],
        vec![5, 2],
        vec![4, 1, 2, 2],
        vec![2, 6],
    ];
    let mut solver = solver::NonogramSolver::new(15, 15);
    solver.set_row_clues(&row_clues);
    solver.set_col_clues(&col_clues);
    solver.show_state();
    solver.solve();
}
