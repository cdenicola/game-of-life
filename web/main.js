import init, { GameOfLife } from "./pkg/gameoflife.js";

const CELL_SIZE = 16;
const CELL_SPACING = 1;
const GRID_COLOR = "#cbd5f5";
const DEAD_COLOR = "#f8fafc";
const ALIVE_COLOR = "#2563eb";
const WIDTH = 40;
const HEIGHT = 40;

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

const setTickLabel = () => {
  tickLabel.textContent = `${tickInterval} ms`;
};

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;
  ctx.lineWidth = 1;

  for (let i = 0; i <= WIDTH; i += 1) {
    const x = i * (CELL_SIZE + CELL_SPACING) + 0.5;
    ctx.moveTo(x, 0);
    ctx.lineTo(x, canvas.height);
  }

  for (let j = 0; j <= HEIGHT; j += 1) {
    const y = j * (CELL_SIZE + CELL_SPACING) + 0.5;
    ctx.moveTo(0, y);
    ctx.lineTo(canvas.width, y);
  }

  ctx.stroke();
};

const drawCells = () => {
  const cells = game.cells(WIDTH, HEIGHT);
  ctx.beginPath();

  for (let y = 0; y < HEIGHT; y += 1) {
    for (let x = 0; x < WIDTH; x += 1) {
      const idx = y * WIDTH + x;
      ctx.fillStyle = cells[idx] === 1 ? ALIVE_COLOR : DEAD_COLOR;
      ctx.fillRect(
        x * (CELL_SIZE + CELL_SPACING) + CELL_SPACING,
        y * (CELL_SIZE + CELL_SPACING) + CELL_SPACING,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }
};

const render = () => {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  drawCells();
  drawGrid();
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
  for (let y = 0; y < HEIGHT; y += 1) {
    for (let x = 0; x < WIDTH; x += 1) {
      Math.random() < 0.3 ? game.set(x,y) : game.unset(x,y);
    }
  }
  render();
};

const configureCanvas = () => {
  canvas.width = (CELL_SIZE + CELL_SPACING) * WIDTH + CELL_SPACING;
  canvas.height = (CELL_SIZE + CELL_SPACING) * HEIGHT + CELL_SPACING;
};

const cellFromEvent = (event) => {
  const boundingRect = canvas.getBoundingClientRect();
  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const col = Math.min(
    WIDTH - 1,
    Math.floor(canvasLeft / (CELL_SIZE + CELL_SPACING))
  );
  const row = Math.min(
    HEIGHT - 1,
    Math.floor(canvasTop / (CELL_SIZE + CELL_SPACING))
  );

  return { col, row };
};

let isPointerDown = false;
let paintValue = true;

const handlePointerDown = (event) => {
  event.preventDefault();
  isPointerDown = true;
  try {
    canvas.setPointerCapture(event.pointerId);
  } catch (_) {
    // ignore browsers that disallow capture here
  }
  const { col, row } = cellFromEvent(event);
  paintValue = game.toggle(col, row);
  render();
};

const handlePointerMove = (event) => {
  if (!isPointerDown) {
    return;
  }

  event.preventDefault();
  const { col, row } = cellFromEvent(event);
  paintValue ? game.set(col,row) : game.unset(col,row);
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
};

const boot = async () => {
  await init();
  game = GameOfLife.new();
  configureCanvas();
  setTickLabel();
  render();
  wireEvents();
};

boot();
