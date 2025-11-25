from solver import NonogramSolver

row_clues = [
    [6],
    [1,1],
    [1, 1],
    [1, 1,1],
    [1, 6],
    [1, 1, 1],
    [1,5],
    [1],
    [7],
    [1],
]
col_clues = [
    [7,1],
    [1,1],
    [1,1,1],
    [6,1],
    [1,1,1],
    [1,7],
    [1,1,1],
    [1,1],
    [1],
    [1],
]

MySolver = NonogramSolver(
    row=10,
    col=10,
    row_clues=row_clues,
    col_clues=col_clues,
)
MySolver.solve()
MySolver.print_solution()
