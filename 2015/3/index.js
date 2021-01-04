const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("");

const dir = { "<": [-1, 0], ">": [1, 0], "^": [0, -1], v: [0, 1] };
function task1() {
  const coord = new Set();
  let x = 0;
  let y = 0;
  coord.add(`${x};${y}`);
  input.forEach((l) => {
    x += dir[l][0];
    y += dir[l][1];
    coord.add(`${x};${y}`);
  });

  return coord.size;
}

function task2() {
  const coord = new Set();
  let x = 0;
  let y = 0;
  let xx = 0;
  let yy = 0;
  coord.add(`${x};${y}`);
  input.forEach((l, i) => {
    if (i % 2) {
      x += dir[l][0];
      y += dir[l][1];
    } else {
      xx += dir[l][0];
      yy += dir[l][1];
    }
    coord.add(`${x};${y}`);
    coord.add(`${xx};${yy}`);
  });

  return coord.size;
}

console.log(task1());
console.log(task2());
