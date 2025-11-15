import init, { GameOfLife } from "./pkg/gameoflife.js";

const CELL_SPACING = 1;
const GRID_COLOR = "#cbd5f5";
const DEAD_COLOR = "#f8fafc";
const ALIVE_COLOR = "#2563eb";
const MIN_CELL_SIZE = 4;
const MAX_CELL_SIZE = 48;
const ZOOM_FACTOR = 1.2;

const canvas = document.getElementById("board");
const ctx = canvas.getContext("2d");
const toggleRunButton = document.getElementById("toggleRun");
const stepButton = document.getElementById("step");
const clearButton = document.getElementById("clear");
const randomButton = document.getElementById("random");
const tickSlider = document.getElementById("tickRate");
const tickLabel = document.getElementById("tickRateLabel");

let game;
let tickInterval = Number(tickSlider.value);
let intervalId = null;

const viewport = {
  originX: 0,
  originY: 0,
  cellSize: 16,
};

const clamp = (value, min, max) => Math.max(min, Math.min(max, value));

const getBoardDimensions = (cellSize = viewport.cellSize) => {
  const pitch = cellSize + CELL_SPACING;
  return {
    width: Math.max(1, Math.floor(canvas.width / pitch)),
    height: Math.max(1, Math.floor(canvas.height / pitch)),
    pitch,
  };
};

const configureCanvas = () => {
  const rect = canvas.getBoundingClientRect();
  if (rect.width === 0 || rect.height === 0) {
    return;
  }
  canvas.width = Math.floor(rect.width);
  canvas.height = Math.floor(rect.height);
};

const initializeViewport = () => {
  const { width, height } = getBoardDimensions();
  viewport.originX = -Math.floor(width / 2);
  viewport.originY = -Math.floor(height / 2);
};

const setTickLabel = () => {
  tickLabel.textContent = `${tickInterval} ms`;
};

const drawGrid = ({ width, height, pitch }) => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;
  ctx.lineWidth = 1;

  for (let i = 0; i <= width; i += 1) {
    const x = i * pitch + 0.5;
    ctx.moveTo(x, 0);
    ctx.lineTo(x, canvas.height);
  }

  for (let j = 0; j <= height; j += 1) {
    const y = j * pitch + 0.5;
    ctx.moveTo(0, y);
    ctx.lineTo(canvas.width, y);
  }

  ctx.stroke();
};

const drawCells = ({ width, height, pitch }) => {
  const cells = game.cells_at(width, height, viewport.originX, viewport.originY);
  ctx.beginPath();

  for (let y = 0; y < height; y += 1) {
    for (let x = 0; x < width; x += 1) {
      const idx = y * width + x;
      ctx.fillStyle = cells[idx] === 1 ? ALIVE_COLOR : DEAD_COLOR;
      ctx.fillRect(
        x * pitch + CELL_SPACING,
        y * pitch + CELL_SPACING,
        viewport.cellSize,
        viewport.cellSize
      );
    }
  }
};

const render = () => {
  if (!game) {
    return;
  }

  const dims = getBoardDimensions();
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  drawCells(dims);
  drawGrid(dims);
};

const start = () => {
  if (intervalId !== null) {
    return;
  }

  intervalId = setInterval(() => {
    game.tick();
    render();
  }, tickInterval);
  toggleRunButton.textContent = "Pause";
};

const stop = () => {
  if (intervalId !== null) {
    clearInterval(intervalId);
    intervalId = null;
  }
  toggleRunButton.textContent = "Start";
};

const randomizeBoard = () => {
  stop();
  const { width, height } = getBoardDimensions();
  for (let y = 0; y < height; y += 1) {
    for (let x = 0; x < width; x += 1) {
      const worldX = viewport.originX + x;
      const worldY = viewport.originY + y;
      Math.random() < 0.3 ? game.set(worldX, worldY) : game.unset(worldX, worldY);
    }
  }
  render();
};

const cellFromEvent = (event, cellSizeOverride = viewport.cellSize) => {
  const rect = canvas.getBoundingClientRect();
  const scaleX = canvas.width / rect.width;
  const scaleY = canvas.height / rect.height;

  const canvasLeft = (event.clientX - rect.left) * scaleX;
  const canvasTop = (event.clientY - rect.top) * scaleY;
  const dims = getBoardDimensions(cellSizeOverride);

  const col = clamp(Math.floor(canvasLeft / dims.pitch), 0, dims.width - 1);
  const row = clamp(Math.floor(canvasTop / dims.pitch), 0, dims.height - 1);

  return {
    col,
    row,
    worldX: viewport.originX + col,
    worldY: viewport.originY + row,
  };
};

let isPointerDown = false;
let isPanning = false;
let paintValue = true;
let lastPan = { x: 0, y: 0 };
let panRemainderX = 0;
let panRemainderY = 0;
let spacePressed = false;

const shouldPan = (event) => {
  if (event.pointerType === "touch") {
    return event.button !== 0;
  }

  return (
    event.button === 1 ||
    event.button === 2 ||
    event.metaKey ||
    event.ctrlKey ||
    event.altKey ||
    spacePressed
  );
};

const applyPanDelta = (dx, dy) => {
  const { pitch } = getBoardDimensions();
  panRemainderX += dx / pitch;
  panRemainderY += dy / pitch;

  const shiftX = Math.trunc(panRemainderX);
  const shiftY = Math.trunc(panRemainderY);

  if (shiftX !== 0 || shiftY !== 0) {
    viewport.originX -= shiftX;
    viewport.originY -= shiftY;
    panRemainderX -= shiftX;
    panRemainderY -= shiftY;
    render();
  }
};

const handlePointerDown = (event) => {
  event.preventDefault();
  isPointerDown = true;
  isPanning = shouldPan(event) || event.button !== 0;
  try {
    canvas.setPointerCapture(event.pointerId);
  } catch (_) {
    // ignore browsers that disallow capture here
  }

  if (isPanning) {
    lastPan = { x: event.clientX, y: event.clientY };
    panRemainderX = 0;
    panRemainderY = 0;
    return;
  }

  const { worldX, worldY } = cellFromEvent(event);
  paintValue = game.toggle(worldX, worldY);
  render();
};

const handlePointerMove = (event) => {
  if (!isPointerDown) {
    return;
  }

  event.preventDefault();

  if (isPanning) {
    const dx = event.clientX - lastPan.x;
    const dy = event.clientY - lastPan.y;
    lastPan = { x: event.clientX, y: event.clientY };
    applyPanDelta(dx, dy);
    return;
  }

  const { worldX, worldY } = cellFromEvent(event);
  paintValue ? game.set(worldX, worldY) : game.unset(worldX, worldY);
  render();
};

const handlePointerUp = (event) => {
  if (isPointerDown) {
    try {
      canvas.releasePointerCapture(event.pointerId);
    } catch (_) {
      // ignore when no capture is active
    }
  }
  isPointerDown = false;
  isPanning = false;
  panRemainderX = 0;
  panRemainderY = 0;
};

const interactiveKeys = new Set(["INPUT", "TEXTAREA", "BUTTON", "SELECT"]);

const handleSpaceDown = (event) => {
  if (event.code !== "Space") {
    return;
  }
  if (interactiveKeys.has(event.target.tagName)) {
    return;
  }
  spacePressed = true;
  event.preventDefault();
};

const handleSpaceUp = (event) => {
  if (event.code !== "Space") {
    return;
  }
  spacePressed = false;
  if (!interactiveKeys.has(event.target.tagName)) {
    event.preventDefault();
  }
};

const handleWheel = (event) => {
  event.preventDefault();
  const direction = Math.sign(event.deltaY);
  if (direction === 0) {
    return;
  }

  const nextSize = clamp(
    Math.round(
      direction > 0
        ? viewport.cellSize / ZOOM_FACTOR
        : viewport.cellSize * ZOOM_FACTOR
    ),
    MIN_CELL_SIZE,
    MAX_CELL_SIZE
  );

  if (nextSize === viewport.cellSize) {
    return;
  }

  const before = cellFromEvent(event);
  viewport.cellSize = nextSize;
  const after = cellFromEvent(event);

  viewport.originX += before.col - after.col;
  viewport.originY += before.row - after.row;
  render();
};

const wireEvents = () => {
  toggleRunButton.addEventListener("click", () => {
    if (intervalId === null) {
      start();
    } else {
      stop();
    }
  });

  stepButton.addEventListener("click", () => {
    stop();
    game.tick();
    render();
  });

  clearButton.addEventListener("click", () => {
    stop();
    game.clear();
    render();
  });

  randomButton.addEventListener("click", randomizeBoard);

  const handleTickChange = (event) => {
    tickInterval = Number(event.target.value);
    setTickLabel();
    if (intervalId !== null) {
      stop();
      start();
    }
  };

  tickSlider.addEventListener("input", handleTickChange);
  tickSlider.addEventListener("change", handleTickChange);

  canvas.addEventListener("pointerdown", handlePointerDown);
  canvas.addEventListener("pointermove", handlePointerMove);
  canvas.addEventListener("pointerup", handlePointerUp);
  canvas.addEventListener("pointerleave", handlePointerUp);
  canvas.addEventListener("pointercancel", handlePointerUp);
  canvas.addEventListener("wheel", handleWheel, { passive: false });
  canvas.addEventListener("contextmenu", (event) => event.preventDefault());

  window.addEventListener("keydown", handleSpaceDown);
  window.addEventListener("keyup", handleSpaceUp);
  window.addEventListener("resize", () => {
    configureCanvas();
    render();
  });
};

const boot = async () => {
  await init();
  game = GameOfLife.new();
  configureCanvas();
  initializeViewport();
  setTickLabel();
  render();
  wireEvents();
};

boot();
