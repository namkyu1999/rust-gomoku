import { Universe, Cell } from "rust-gomoku";
import { memory } from "rust-gomoku/rust_gomoku_bg";

const GRID_SIZE = 38;
const CELL_SIZE = 17; // px
const UNIVERSE_COLOR = "#f0b060";
const GRID_COLOR = "#956039";
const WHITE_COLOR = "#f3f3f3";
const BLACK_COLOR = "#282828";

const INIT_POINT = 77;

const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById("gomoku-canvas");

var is_win = Cell.None;

canvas.height = 700;
canvas.width = 700;

const ctx = canvas.getContext('2d');

const drawGrid = () => {
    ctx.beginPath();

    ctx.fillStyle = UNIVERSE_COLOR;
    ctx.fillRect(0,0,800,800);
    ctx.closePath();
    ctx.fill();

    ctx.strokeStyle = GRID_COLOR;
  
    // Vertical lines.
    for (let i = 0; i < width; i++) {
      ctx.moveTo(INIT_POINT + i * (GRID_SIZE + 1) + 1, INIT_POINT);
      ctx.lineTo(INIT_POINT + i * (GRID_SIZE + 1) + 1, INIT_POINT + (GRID_SIZE + 1) * (height - 1) + 1);
    }
  
    // Horizontal lines.
    for (let j = 0; j < height; j++) {
      ctx.moveTo(INIT_POINT,                               INIT_POINT + j * (GRID_SIZE + 1) + 1);
      ctx.lineTo(INIT_POINT + (GRID_SIZE + 1) * (width - 1) + 1, INIT_POINT + j * (GRID_SIZE + 1) + 1);
    }
  
    ctx.stroke();

};

const getIndex = (row, column) => {
  return row * width + column;
};

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {

      ctx.beginPath();
      ctx.strokeStyle = BLACK_COLOR;

      const idx = getIndex(row, col);

      if (cells[idx] === Cell.None) continue;
      
      ctx.fillStyle = cells[idx] === Cell.White
          ? WHITE_COLOR
          : BLACK_COLOR;

      ctx.moveTo(
        INIT_POINT + col * (GRID_SIZE + 1) + 1 + CELL_SIZE,
        INIT_POINT + row * (GRID_SIZE + 1) + 1
      );
      
      ctx.arc(
        INIT_POINT + col * (GRID_SIZE + 1) + 1 ,
        INIT_POINT + row * (GRID_SIZE + 1) + 1,
        CELL_SIZE,
        0, 
        Math.PI * 2, 
        true
      );

      ctx.fill();
      ctx.stroke();
    }
  }
};

canvas.addEventListener("click", event => {

  if(is_win === Cell.None){
    
    const x = event.layerX;
    const y = event.layerY;

    const row = Math.round(y-57.5) / (GRID_SIZE+1);
    const col = Math.round(x-57.5) / (GRID_SIZE+1);

    var validation = 57.5> x || x > 642.5 || 57.5> y || y > 642.5 ;
    
    if(universe.is_non(row,col) && !validation){

      universe.toggle(row,col);

      is_win = universe.is_win();

      drawGrid();
      drawCells();

    }
  }
  
});

drawGrid();
drawCells();
