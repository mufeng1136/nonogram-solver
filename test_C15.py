# test_C15.py

from solver import NonogramSolver
from solver import NonogramSolverV2

row_clues = [
    [6,1,2],
    [5,1,3],
    [8,5],
    [1,3,1,5],
    [9],

    [1,1,6],
    [1,1,2,1],
    [1,3,1,1],
    [2,1],
    [6,2],

    [2,7],
    [2,5],
    [6,1],
    [3,1,1],
    [3,1,1],

]
col_clues = [
    [4,3],
    [3,3],
    [4,1],
    [4,3,1],
    [5,1,2],

    [1,1,4,3],
    [1,1,1,1,2],
    [4,1,1,1],
    [1,3],
    [6,3],

    [6,3],
    [5,6],
    [5,2],
    [4,1,2,2],
    [2,6],

]

MySolver = NonogramSolverV2(
    row=15,
    col=15,
    row_clues=row_clues,
    col_clues=col_clues,
)
MySolver.solve()
MySolver.print_solution()
