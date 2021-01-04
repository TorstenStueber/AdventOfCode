const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((line) => {
    const [_, fixed, at, from, to] = /(x|y)=(\d+), .=(\d+)\.\.(\d+)/.exec(line);
    return {
      fixed,
      at: Number(at),
      from: Number(from),
      to: Number(to),
    };
  });

let minX = Infinity;
let maxX = -Infinity;
let minY = Infinity;
let maxY = -Infinity;

input.forEach(({ fixed, at, from, to }) => {
  if (fixed === "x") {
    minX = Math.min(minX, at);
    maxX = Math.max(maxX, at);
    minY = Math.min(minY, from);
    maxY = Math.max(maxY, to);
  } else {
    minX = Math.min(minX, from);
    maxX = Math.max(maxX, to);
    minY = Math.min(minY, at);
    maxY = Math.max(maxY, at);
  }
});

const map = [];
for (let y = minY; y <= maxY; y++) {
  map.push([]);
  for (let x = minX; x <= maxX; x++) {
    map[y - minY][x - minX] = ".";
  }
}

input.forEach(({ fixed, at, from, to }) => {
  if (fixed === "x") {
    for (let y = from; y <= to; y++) map[y - minY][at - minX] = "#";
  } else {
    for (let x = from; x <= to; x++) map[at - minY][x - minX] = "#";
  }
});

function fill() {
  const startX = 500;
  const startY = minY;
  const touched = new Set();
  const standing = new Set();
  const todo = [[startX, startY]];

  while (todo.length) {
    let [x, y] = todo.shift();
    while (!touched.has(`${x};${y}`)) {
      touched.add(`${x};${y}`);
      if (y + 1 > maxY) break;
      if (map[y - minY + 1][x - minX] === "#" || standing.has(`${x};${y + 1}`)) {
        let leftDrop = false;
        let rightDrop = false;
        while (!leftDrop && !rightDrop) {
          const rowX = new Set();
          for (let xx = x; map[y - minY][xx - minX] !== "#"; xx--) {
            if (map[y - minY + 1][xx - minX] !== "#" && !standing.has(`${xx};${y + 1}`)) {
              leftDrop = true;
              todo.push([xx, y]);
              break;
            }
            touched.add(`${xx};${y}`);
            rowX.add(xx);
          }

          for (let xx = x; map[y - minY][xx - minX] !== "#"; xx++) {
            if (map[y - minY + 1][xx - minX] !== "#" && !standing.has(`${xx};${y + 1}`)) {
              rightDrop = true;
              todo.push([xx, y]);
              break;
            }
            touched.add(`${xx};${y}`);
            rowX.add(xx);
          }

          if (!leftDrop && !rightDrop) {
            Array.from(rowX).forEach((xx) => {
              standing.add(`${xx};${y}`);
            });
            y--;
          }
        }
        break;
      }
      y++;
    }
  }

  return { touched, standing };
}

function task1() {
  return fill().touched.size;
}
function task2() {
  return fill().standing.size;
}

console.log(task1());
console.log(task2());
