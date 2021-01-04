const { strict } = require("assert");
const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function init() {
  let black = new Set();
  input.forEach((line) => {
    let x = 0;
    let y = 0;
    for (let i = 0; i < line.length; ) {
      const dir =
        i < line.length - 1 && (line[i] === "s" || line[i] === "n") && (line[i + 1] === "e" || line[i + 1] === "w")
          ? line.slice(i, i + 2)
          : line.slice(i, i + 1);
      i += dir.length;
      switch (dir) {
        case "ne":
          y--;
          x++;
          break;
        case "se":
          y++;
          break;
        case "nw":
          y--;
          break;
        case "sw":
          y++;
          x--;
          break;
        case "e":
          x++;
          break;
        case "w":
          x--;
          break;
      }
    }
    const key = `${x};${y}`;
    if (black.has(key)) {
      black.delete(key);
    } else {
      black.add(key);
    }
  });

  return black;
}

function task1() {
  return init().size;
}

const neighbors = [
  [-1, 0],
  [1, 0],
  [-1, 1],
  [0, 1],
  [0, -1],
  [1, -1],
];

function task2() {
  let black = init();
  for (let i = 0; i < 100; i++) {
    const newBlack = new Set();
    let minX = Infinity;
    let maxX = -Infinity;
    let minY = Infinity;
    let maxY = -Infinity;
    Array.from(black).forEach((c) => {
      const [x, y] = c.split(";");
      minX = Math.min(minX, Number(x));
      minY = Math.min(minY, Number(y));
      maxX = Math.max(maxX, Number(x));
      maxY = Math.max(maxY, Number(y));
    });

    for (let x = minX - 1; x <= maxX + 1; x++) {
      for (let y = minY - 1; y <= maxY + 1; y++) {
        const c = neighbors.reduce((a, [xx, yy]) => a + (black.has(`${x + xx};${y + yy}`) ? 1 : 0), 0);
        const key = `${x};${y}`;
        if (black.has(key)) {
          if (c === 1 || c === 2) newBlack.add(key);
        } else {
          if (c === 2) newBlack.add(key);
        }
      }
    }

    black = newBlack;
  }

  return black.size;
}

console.log(task1());
console.log(task2());
