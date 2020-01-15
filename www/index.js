import { Universe, Cell } from "burst";
import { memory } from "burst/burst_bg";

// Cell config
const CELL_SIZE = 5;

const CELL_COLORS = {
  0: '#f0f0f0',
  1: '#f1ca88',
  2: '#3147ba'
};

const universe = Universe.new(128, 128);

setInterval(() => {

  for (let i = 0; i < 200; i++) {
    let x = Math.floor(Math.random() * 30);
    let y = Math.floor(Math.random() * 100);

    universe.set_cell(x, y, Cell.Sand);
  }

}, 3000);

setInterval(() => {

  for (let i = 0; i < 100; i++) {
    let x = 60;
    let y = Math.floor(Math.random() * 60);

    universe.set_cell(x, y, Cell.Sand);
  }

}, 4000);

setInterval(() => {

  for (let i = 0; i < 100; i++) {
    let x = 40;
    let y = Math.floor(Math.random() * 60);

    universe.set_cell(x, y, Cell.Water);
  }

}, 1000);

const width = universe.width();
const height = universe.height();

const canvasEl = document.createElement('canvas');
canvasEl.height = (CELL_SIZE + 1) * height + 1;
canvasEl.width = (CELL_SIZE + 1) * width + 1;

document.body.appendChild(canvasEl);

const ctx = canvasEl.getContext('2d');

const getIndex = (row, column) => {
  return row * width + column;
};

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = CELL_COLORS[ cells[idx] ];

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};


const renderLoop = () => {
  universe.tick();

  drawCells();

  requestAnimationFrame(renderLoop);
};

drawCells();
requestAnimationFrame(renderLoop);