# Nonogram Solver

[中文文档](#项目简介)

## Introduction

Nonogram Solver is a Python project for solving Nonograms (Picross). It implements a solver based on a backtracking algorithm with the following features:

- Validates input clues.
- Automatically generates all possible row configurations.
- Uses a recursive backtracking algorithm to try row by row, ensuring column states match column clues.
- Supports visual output of the solution.
- Provides progress logging, showing the current possibility index and total possibilities.

## Environment Requirements

- Python version >= 3.11
- This project uses [uv](https://github.com/astral-sh/uv) for dependency management and execution.

## Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/worldedge1933/nonogram-solver.git
    cd nonogram-solver
    ```

2. Install dependencies using uv:

    ```bash
    uv sync
    ```

## Usage

### Example Code

The project includes an example file `example.py`, which can be run directly:

```bash
uv run example.py
```

### Custom Input

You can use the `NonogramSolver` class to solve custom Nonograms:

```python
from solver.solver import NonogramSolver

# Define row and column clues
row_clues = [[6], [1, 1], [1, 1], [1, 1, 1], [1, 6], [1, 1, 1], [1, 5], [1], [7], [1]]
col_clues = [[1], [1, 1], [6], [1], [1], [7], [1], [1], [1, 1], [1]]

# Initialize the solver
solver = NonogramSolver(row=10, col=10, row_clues=row_clues, col_clues=col_clues)

# Solve
solver.solve()

# Print the solution
solver.print_solution()
```

### Testing

Run tests to verify functionality:

```bash
uv run pytest tests/
```

## Project Structure

```text
nonogram-solver/
├── solver/               # Solver module
│   ├── solver.py         # Implementation of NonogramSolver class
├── example.py            # Example code
├── README.md             # Project documentation
└── pyproject.toml        # Project configuration file
```

## License

MIT License

---

## Nonogram Solver (中文)

## 项目简介

Nonogram Solver 是一个用于解决 Nonogram（数织）的 Python 项目。它实现了一个基于回溯算法的求解器，支持以下功能：

- 验证输入的线索（clues）是否合法。
- 自动生成所有可能的行配置。
- 使用递归回溯算法逐行尝试，确保列状态与列线索匹配。
- 支持打印解决方案的可视化输出。
- 提供进度日志，显示当前尝试的可能性和总可能性。

## 环境建议

- Python 版本 >= 3.11
- 本项目使用 [uv](https://github.com/astral-sh/uv) 进行依赖管理和运行。

## 安装

1. 克隆项目到本地：

    ```bash
    git clone https://github.com/worldedge1933/nonogram-solver.git
    cd nonogram-solver
    ```

2. 使用 uv 安装依赖：

    ```bash
    uv sync
    ```

## 使用方法

### 示例代码

项目包含一个示例文件 `example.py`，可以直接运行：

```bash
uv run example.py
```

### 自定义输入

您可以通过以下方式使用 `NonogramSolver` 类解决自定义的 Nonogram：

```python
from solver.solver import NonogramSolver

# 定义行线索和列线索
row_clues = [[6], [1, 1], [1, 1], [1, 1, 1], [1, 6], [1, 1, 1], [1, 5], [1], [7], [1]]
col_clues = [[1], [1, 1], [6], [1], [1], [7], [1], [1], [1, 1], [1]]

# 初始化求解器
solver = NonogramSolver(row=10, col=10, row_clues=row_clues, col_clues=col_clues)

# 求解
solver.solve()

# 打印解决方案
solver.print_solution()
```

### 测试

运行测试以验证功能：

```bash
uv run pytest tests/
```

## 项目结构

```text
nonogram-solver/
├── solver/               # 求解器模块
│   ├── solver.py         # NonogramSolver 类的实现
├── example.py            # 示例代码
├── README.md             # 项目说明文件
└── pyproject.toml        # 项目配置文件
```


## 许可证

MIT License
