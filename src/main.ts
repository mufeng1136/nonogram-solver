import { invoke } from '@tauri-apps/api/core';

type SolveResponse = {
  rows: number;
  cols: number;
  grid: number[][]; // 0 empty, 1 filled, 2 unknown
  valid: boolean;
  solved: boolean;
  unsolvable: boolean;
};

function cellText(v: number): string {
  if (v === 1) return '■';
  if (v === 0) return '·';
  return '?';
}

function renderGrid(container: HTMLElement, resp: SolveResponse) {
  const table = document.createElement('table');
  table.style.borderCollapse = 'collapse';
  table.style.fontFamily = 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace';

  for (let r = 0; r < resp.rows; r++) {
    const tr = document.createElement('tr');
    for (let c = 0; c < resp.cols; c++) {
      const td = document.createElement('td');
      td.textContent = cellText(resp.grid[r][c]);
      td.style.width = '22px';
      td.style.height = '22px';
      td.style.textAlign = 'center';
      td.style.border = '1px solid #ddd';
      td.style.userSelect = 'none';
      tr.appendChild(td);
    }
    table.appendChild(tr);
  }

  container.innerHTML = '';
  container.appendChild(table);
}

const app = document.getElementById('app');
if (!app) throw new Error('Missing #app');

app.innerHTML = `
  <div style="padding:16px; max-width: 900px; margin: 0 auto;">
    <h1 style="margin:0 0 12px 0; font-size:20px;">Nonogram Solver (Tauri)</h1>
    <div style="display:flex; gap:12px; align-items:center; margin-bottom: 12px;">
      <button id="solve" style="padding:8px 12px;">Solve sample (Rust)</button>
      <span id="status" style="color:#555;"></span>
    </div>
    <div id="grid"></div>
  </div>
`;

const btn = document.getElementById('solve') as HTMLButtonElement;
const status = document.getElementById('status') as HTMLSpanElement;
const grid = document.getElementById('grid') as HTMLDivElement;

btn.addEventListener('click', async () => {
  status.textContent = 'Solving...';
  try {
    const resp = await invoke<SolveResponse>('solve_sample');
    status.textContent = `valid=${resp.valid} solved=${resp.solved} unsolvable=${resp.unsolvable}`;
    renderGrid(grid, resp);
  } catch (e) {
    status.textContent = `Error: ${String(e)}`;
    grid.innerHTML = '';
  }
});
