import { invoke } from '@tauri-apps/api/core';

type Lang = 'zh' | 'en';

function loadLang(): Lang {
  const v = localStorage.getItem('lang');
  return v === 'en' ? 'en' : 'zh';
}

function saveLang(lang: Lang) {
  localStorage.setItem('lang', lang);
}

const i18n = {
  zh: {
    appTitle: '数织求解器',
    appTitleDocument: '数织求解器',
    instructions: '线索输入：每行/列使用“空格或逗号分隔的正整数”；留空表示该行/列没有填充块。',
    rows: '行',
    cols: '列',
    applySize: '应用尺寸',
    puzzle: '谜题',
    solve: '开始求解',
    cellClickHint: '点击格子可在「未知 → 填充 → 空」之间循环切换；编辑后会清除求解结果。',
    clueHintDefault: '点击线索进行编辑',
    cluePlaceholder: '例如：3 1 2（留空表示无块）',
    navHint: '回车：下一行/列；Shift+回车：上一行/列。',
    cornerTitle: '列线索在上，行线索在左',
    editingRow: (n: number) => `正在编辑第 ${n} 行`,
    editingCol: (n: number) => `正在编辑第 ${n} 列`,
    cellUnknown: '未知',
    cellFilled: '填充',
    cellEmpty: '空',
    sizeInvalid: '尺寸无效（范围 1..60）。',
    sizeSet: (r: number, c: number) => `尺寸已设置为 ${r}×${c}`,
    solving: '求解中…',
    done: '完成。',
    solved: '已求解。',
    clueInvalid: '线索不合法',
    unsolvable: '无解',
    notSolved: '未解',
    tauriUnavailable:
      '无法调用 Tauri API：你可能是在普通浏览器中打开了前端页面。' +
      '请使用 `cargo tauri dev` 启动（开发模式），或运行打包后的可执行文件（`src-tauri/target/release` 或 bundle 输出）。',
    missingApp: '缺少 #app 元素',
    clueFormatError: (text: string) => `线索格式错误："${text}"（请使用空格或逗号分隔的正整数）`,
    langToggle: 'English',
  },
  en: {
    appTitle: 'Nonogram Solver',
    appTitleDocument: 'Nonogram Solver',
    instructions:
      'Clues: use positive integers separated by space/comma for each row/column; leave blank for “no filled blocks”.',
    rows: 'Rows',
    cols: 'Cols',
    applySize: 'Apply size',
    puzzle: 'Puzzle',
    solve: 'Solve',
    cellClickHint: 'Click a cell to cycle: unknown → filled → empty (repeat). Editing clears the solved result.',
    clueHintDefault: 'Click a clue to edit',
    cluePlaceholder: 'e.g. 3 1 2 (empty = none)',
    navHint: 'Enter: next row/col; Shift+Enter: previous row/col.',
    cornerTitle: 'Column clues above, row clues left',
    editingRow: (n: number) => `Editing row R${n}`,
    editingCol: (n: number) => `Editing column C${n}`,
    cellUnknown: 'unknown',
    cellFilled: 'filled',
    cellEmpty: 'empty',
    sizeInvalid: 'Invalid size (1..60).',
    sizeSet: (r: number, c: number) => `Size set to ${r}x${c}`,
    solving: 'Solving...',
    done: 'Done.',
    solved: 'Solved.',
    clueInvalid: 'Invalid clues',
    unsolvable: 'Unsolvable',
    notSolved: 'Not solved',
    tauriUnavailable:
      'Tauri API not available. You are likely opening the frontend in a normal browser. ' +
      'Please launch it via `cargo tauri dev` (dev) or run the built executable from `src-tauri/target/release` / the bundle output.',
    missingApp: 'Missing #app',
    clueFormatError: (text: string) => `Invalid clue line: "${text}" (use positive integers separated by space/comma)`,
    langToggle: '中文',
  },
} as const;

let lang: Lang = loadLang();

function t<K extends keyof (typeof i18n)['zh']>(key: K): (typeof i18n)['zh'][K] {
  return (i18n as any)[lang][key];
}

// 注意：Tauri API 只有在 Tauri WebView 内加载页面时才可用。
// 如果你在普通浏览器里直接打开页面（例如通过 Vite URL 或 dist/index.html），注入的全局对象不会存在。

type SolveResponse = {
  rows: number;
  cols: number;
  grid: number[][]; // 0 空，1 填充，2 未知
  valid: boolean;
  solved: boolean;
  unsolvable: boolean;
};

type SolveRequest = {
  rows: number;
  cols: number;
  rowClues: number[][];
  colClues: number[][];
  knownGrid?: number[][];
};

function isTauriRuntime(): boolean {
  const w = window as any;
  return typeof w?.__TAURI_INTERNALS__?.invoke === 'function' || typeof w?.__TAURI__?.core?.invoke === 'function';
}

async function tauriInvoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  if (!isTauriRuntime()) {
    throw new Error(t('tauriUnavailable'));
  }
  return (await invoke<T>(command, args)) as T;
}

function cellLabel(v: number): string {
  if (v === 1) return '';
  if (v === 0) return '';
  return '';
}

function cellBg(v: number): string {
  if (v === 1) return '#93c5fd'; // 填充（浅蓝）
  if (v === 0) return '#ffffff'; // 空
  return '#f3f4f6'; // 未知
}

function cellBorder(v: number): string {
  if (v === 1) return '#60a5fa';
  if (v === 0) return '#d1d5db';
  return '#d1d5db';
}

function splitClueTokens(text: string): string[] {
  const normalized = text.replace(/，/g, ',').trim();
  if (!normalized) return [];
  return normalized.split(/[\s,]+/).filter(Boolean);
}

function parseClueLine(text: string): number[] {
  const parts = splitClueTokens(text);
  if (parts.length === 0) return [];
  const nums = parts.map((p) => Number(p));
  if (nums.some((n) => !Number.isFinite(n) || !Number.isInteger(n) || n <= 0)) {
    throw new Error(t('clueFormatError')(text));
  }
  return nums;
}

function createGrid(rows: number, cols: number, fill: number): number[][] {
  return Array.from({ length: rows }, () => Array.from({ length: cols }, () => fill));
}

function clueDisplay(text: string): string {
  const t = text.trim();
  return t ? t : '·';
}

function clueDisplayVertical(text: string): string {
  const t = text.trim();
  if (!t) return '·';
  try {
    return parseClueLine(t)
      .map((n) => String(n))
      .join('\n');
  } catch {
    return splitClueTokens(t).join('\n');
  }
}

const appEl = document.getElementById('app');
if (!appEl) throw new Error(t('missingApp'));
const app = appEl;

const state = {
  rows: 10,
  cols: 10,
  rowClueText: [] as string[],
  colClueText: [] as string[],
  knownGrid: [] as number[][],
  solvedGrid: null as number[][] | null,
};

function initForSize(rows: number, cols: number) {
  state.rows = rows;
  state.cols = cols;
  state.rowClueText = Array.from({ length: rows }, () => '');
  state.colClueText = Array.from({ length: cols }, () => '');
  state.knownGrid = createGrid(rows, cols, 2);
  state.solvedGrid = null;
}

initForSize(state.rows, state.cols);

let rowsInput!: HTMLInputElement;
let colsInput!: HTMLInputElement;
let applyBtn!: HTMLButtonElement;
let solveBtn!: HTMLButtonElement;
let statusEl!: HTMLSpanElement;
let puzzleStatusEl!: HTMLSpanElement;
let editorEl!: HTMLDivElement;
let clueHintEl!: HTMLDivElement;
let clueInputEl!: HTMLInputElement;
let langToggleBtn!: HTMLButtonElement;

function setPuzzleStatus(msg: string) {
  puzzleStatusEl.textContent = msg;
}

function invalidateSolveResult() {
  state.solvedGrid = null;
  setPuzzleStatus('');
}

function renderEditor() {
  const cellSize = 22;
  const table = document.createElement('table');
  table.style.borderCollapse = 'collapse';
  table.style.tableLayout = 'fixed';
  table.style.width = 'fit-content';
  (table.style as any).display = 'inline-table';

  const clueCellStyle = (td: HTMLTableCellElement) => {
    td.style.border = '1px solid #e5e7eb';
    td.style.background = '#ffffff';
    td.style.color = '#111827';
    td.style.fontSize = '12px';
    td.style.padding = '0';
    td.style.height = '22px';
    td.style.userSelect = 'none';
    td.style.boxSizing = 'border-box';
  };

  const getMaxColClueLines = () => {
    let max = 1;
    for (let c = 0; c < state.cols; c++) {
      const t = (state.colClueText[c] ?? '').trim();
      if (!t) continue;
      const parts = splitClueTokens(t);
      max = Math.max(max, parts.length);
    }
    return max;
  };
  const colClueHeight = Math.max(2, Math.min(8, getMaxColClueLines())) * cellSize;

  const setClueContent = (td: HTMLTableCellElement, text: string, vertical: boolean) => {
    td.textContent = vertical ? clueDisplayVertical(text) : clueDisplay(text);
    td.style.textAlign = 'center';
    td.style.verticalAlign = vertical ? 'bottom' : 'middle';
    td.style.whiteSpace = 'pre-line';
    td.style.overflow = 'hidden';
    td.style.textOverflow = 'ellipsis';
    td.style.lineHeight = '1.05';
  };

  // Header row: corner + column clues
  {
    const tr = document.createElement('tr');
    const corner = document.createElement('td');
    clueCellStyle(corner);
    corner.style.width = '90px';
    corner.style.height = `${colClueHeight}px`;
    corner.style.minWidth = corner.style.width;
    corner.style.maxWidth = corner.style.width;
    corner.style.minHeight = corner.style.height;
    corner.style.maxHeight = corner.style.height;
    corner.style.background = '#f9fafb';
    corner.title = t('cornerTitle');
    tr.appendChild(corner);

    for (let c = 0; c < state.cols; c++) {
      const td = document.createElement('td');
      clueCellStyle(td);
      td.style.width = `${cellSize}px`;
      td.style.height = `${colClueHeight}px`;
      td.style.minWidth = td.style.width;
      td.style.maxWidth = td.style.width;
      td.style.minHeight = td.style.height;
      td.style.maxHeight = td.style.height;
      td.style.background = '#f9fafb';
      td.style.cursor = 'pointer';
      setClueContent(td, state.colClueText[c] ?? '', true);
      td.addEventListener('click', () => {
        editing.kind = 'col';
        editing.index = c;
        clueHintEl.textContent = t('editingCol')(c + 1);
        clueInputEl.disabled = false;
        clueInputEl.value = state.colClueText[c] ?? '';
        clueInputEl.focus();
        clueInputEl.select();
      });
      tr.appendChild(td);
    }

    table.appendChild(tr);
  }

  // Each row: row clue + grid cells
  for (let r = 0; r < state.rows; r++) {
    const tr = document.createElement('tr');

    const rowClueTd = document.createElement('td');
    clueCellStyle(rowClueTd);
    rowClueTd.style.width = '90px';
    rowClueTd.style.minWidth = rowClueTd.style.width;
    rowClueTd.style.maxWidth = rowClueTd.style.width;
    rowClueTd.style.cursor = 'pointer';
    rowClueTd.style.background = '#f9fafb';

    setClueContent(rowClueTd, state.rowClueText[r] ?? '', false);
    rowClueTd.style.paddingRight = '6px';
    rowClueTd.style.textAlign = 'right';
    rowClueTd.addEventListener('click', () => {
      editing.kind = 'row';
      editing.index = r;
      clueHintEl.textContent = t('editingRow')(r + 1);
      clueInputEl.disabled = false;
      clueInputEl.value = state.rowClueText[r] ?? '';
      clueInputEl.focus();
      clueInputEl.select();
    });
    tr.appendChild(rowClueTd);

    for (let c = 0; c < state.cols; c++) {
      const v = state.solvedGrid ? state.solvedGrid[r][c] : state.knownGrid[r][c];
      const td = document.createElement('td');
      td.textContent = cellLabel(v);
      td.style.width = `${cellSize}px`;
      td.style.height = `${cellSize}px`;
      td.style.minWidth = td.style.width;
      td.style.maxWidth = td.style.width;
      td.style.minHeight = td.style.height;
      td.style.maxHeight = td.style.height;
      td.style.boxSizing = 'border-box';
      td.style.border = `1px solid ${cellBorder(v)}`;
      td.style.background = cellBg(v);
      td.style.cursor = 'pointer';
      td.title = v === 2 ? t('cellUnknown') : v === 1 ? t('cellFilled') : t('cellEmpty');
      td.addEventListener('click', () => {
        invalidateSolveResult();
        const cur = state.knownGrid[r][c];
        const next = cur === 2 ? 1 : cur === 1 ? 0 : 2;
        state.knownGrid[r][c] = next;
        renderEditor();
      });
      tr.appendChild(td);
    }

    table.appendChild(tr);
  }

  editorEl.innerHTML = '';
  editorEl.appendChild(table);
}

const editing: { kind: 'row' | 'col' | null; index: number } = { kind: null, index: -1 };

function commitCurrentClue() {
  if (!editing.kind) return;
  const value = clueInputEl.value;
  invalidateSolveResult();
  if (editing.kind === 'row') state.rowClueText[editing.index] = value;
  else state.colClueText[editing.index] = value;
}

function advanceClue(delta: number) {
  if (!editing.kind) return;
  const max = editing.kind === 'row' ? state.rows : state.cols;
  const next = editing.index + delta;
  if (next < 0 || next >= max) {
    clueInputEl.blur();
    return;
  }
  editing.index = next;
  clueHintEl.textContent = editing.kind === 'row' ? t('editingRow')(next + 1) : t('editingCol')(next + 1);
  clueInputEl.value = editing.kind === 'row' ? (state.rowClueText[next] ?? '') : (state.colClueText[next] ?? '');
  renderEditor();
  clueInputEl.focus();
  clueInputEl.select();
}

function mountUI() {
  document.title = t('appTitleDocument');

  app.innerHTML = `
    <div style="padding:16px; max-width: 1100px; margin: 0 auto; font-family: ui-sans-serif, system-ui, -apple-system, Segoe UI, Roboto, Helvetica, Arial;">
      <div style="display:flex; align-items:flex-start; justify-content:space-between; gap:12px;">
        <h1 style="margin:0 0 8px 0; font-size:20px;">${t('appTitle')}</h1>
        <button id="langToggle" style="height:28px; padding:0 10px; border:1px solid #e5e7eb; border-radius:6px; background:#ffffff; color:#111827; cursor:pointer;">
          ${t('langToggle')}
        </button>
      </div>

      

      <div style="display:flex; gap:12px; align-items:end; flex-wrap: wrap; margin-bottom: 16px;">
        <div>
          <label style="display:block; font-size:12px; color:#374151;">${t('rows')}</label>
          <input id="rows" type="number" min="1" max="60" value="${state.rows}" style="width:90px; height:32px; padding:6px 8px; box-sizing:border-box;" />
        </div>
        <div>
          <label style="display:block; font-size:12px; color:#374151;">${t('cols')}</label>
          <input id="cols" type="number" min="1" max="60" value="${state.cols}" style="width:90px; height:32px; padding:6px 8px; box-sizing:border-box;" />
        </div>
        <button id="applySize" style="height:32px; padding:0 12px;">${t('applySize')}</button>
        <span id="status" style="color:#374151; font-size:13px;"></span>
      </div>

      <div style="margin-top: 8px;">
        <div style="display:flex; align-items:center; gap:8px; margin:0 0 8px 0;">
          <div style="font-size:14px; font-weight:600; color:#111827; line-height:20px;">${t('puzzle')}</div>
          <button id="solve" style="height:24px; padding:0 10px; background:#111827; color:#fff; border: 1px solid #111827; border-radius:6px;">${t('solve')}</button>
          <span id="puzzleStatus" style="font-size:12px; color:#374151;"></span>
        </div>

        <div style="margin:0 0 8px 0; font-size:12px; color:#6b7280;">${t('cellClickHint')}</div>

        <div id="editor" style="display:inline-block; overflow:auto; max-width:100%; border:1px solid #e5e7eb; border-radius: 6px; padding: 8px;"></div>
        <div id="clueEditor" style="margin-top: 10px; display:flex; gap: 8px; align-items:center;">
          <div id="clueHint" style="min-width: 140px; font-size:12px; color:#374151;">${t('clueHintDefault')}</div>
          <input id="clueInput" type="text" placeholder="${t('cluePlaceholder')}" style="width: 270px; padding:6px 8px; height:32px; box-sizing:border-box;" disabled />
        </div>
        <div style="color:#6b7280; margin-bottom:12px; margin-top:12px; font-size: 13px;">${t('instructions')}</div>
        <div style="margin-top:6px; font-size:12px; color:#6b7280;">${t('navHint')}</div>
      </div>
    </div>
  `;

  langToggleBtn = document.getElementById('langToggle') as HTMLButtonElement;
  rowsInput = document.getElementById('rows') as HTMLInputElement;
  colsInput = document.getElementById('cols') as HTMLInputElement;
  applyBtn = document.getElementById('applySize') as HTMLButtonElement;
  solveBtn = document.getElementById('solve') as HTMLButtonElement;
  statusEl = document.getElementById('status') as HTMLSpanElement;
  puzzleStatusEl = document.getElementById('puzzleStatus') as HTMLSpanElement;
  editorEl = document.getElementById('editor') as HTMLDivElement;
  clueHintEl = document.getElementById('clueHint') as HTMLDivElement;
  clueInputEl = document.getElementById('clueInput') as HTMLInputElement;

  langToggleBtn.addEventListener('click', () => {
    lang = lang === 'zh' ? 'en' : 'zh';
    saveLang(lang);
    mountUI();
  });

  clueInputEl.addEventListener('keydown', (e) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      commitCurrentClue();
      advanceClue(e.shiftKey ? -1 : 1);
    }
  });

  clueInputEl.addEventListener('blur', () => {
    commitCurrentClue();
    renderEditor();
  });

  applyBtn.addEventListener('click', () => {
    const rows = Number(rowsInput.value);
    const cols = Number(colsInput.value);
    if (!Number.isInteger(rows) || rows <= 0 || rows > 60 || !Number.isInteger(cols) || cols <= 0 || cols > 60) {
      statusEl.textContent = t('sizeInvalid');
      return;
    }
    initForSize(rows, cols);
    renderEditor();
    setPuzzleStatus('');
    clueHintEl.textContent = t('clueHintDefault');
    clueInputEl.disabled = true;
    clueInputEl.value = '';
    editing.kind = null;
    editing.index = -1;
    statusEl.textContent = t('sizeSet')(rows, cols);
  });

  solveBtn.addEventListener('click', async () => {
    statusEl.textContent = t('solving');
    state.solvedGrid = null;
    setPuzzleStatus('');

    try {
      const rowClues = state.rowClueText.map(parseClueLine);
      const colClues = state.colClueText.map(parseClueLine);

      const request: SolveRequest = {
        rows: state.rows,
        cols: state.cols,
        rowClues,
        colClues,
        knownGrid: state.knownGrid,
      };

      const resp = await tauriInvoke<SolveResponse>('solve', { request });
      if (!resp.valid) {
        state.solvedGrid = null;
        setPuzzleStatus(t('clueInvalid'));
        statusEl.textContent = t('done');
      } else if (resp.unsolvable) {
        state.solvedGrid = null;
        setPuzzleStatus(t('unsolvable'));
        statusEl.textContent = t('done');
      } else if (resp.solved) {
        state.solvedGrid = resp.grid;
        setPuzzleStatus(t('solved'));
        statusEl.textContent = t('solved');
      } else {
        state.solvedGrid = null;
        setPuzzleStatus(t('notSolved'));
        statusEl.textContent = t('done');
      }

      renderEditor();
    } catch (e) {
      statusEl.textContent = String(e);
    }
  });

  renderEditor();
  setPuzzleStatus('');
}

mountUI();
