const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split(",");

const dirs = {
  ne: [1, -1],
  se: [1, 0],
  s: [0, 1],
  sw: [-1, 1],
  nw: [-1, 0],
  n: [0, -1],
};

function dist(x, y) {
  const maxY = x > 0 ? 0 : -x;
  const minY = x > 0 ? -x : 0;

  if (y >= minY && y <= maxY) return Math.abs(x);
  if (y < minY) return Math.abs(x) + minY - y;
  return Math.abs(x) + y - maxY;
}

function task1() {
  let x = 0;
  let y = 0;
  input.forEach((dir) => {
    const [xx, yy] = dirs[dir];
    x += xx;
    y += yy;
  });
  return dist(x, y);
}

function task2() {
  let x = 0;
  let y = 0;
  let max = 0;
  input.forEach((dir) => {
    const [xx, yy] = dirs[dir];
    x += xx;
    y += yy;
    max = Math.max(max, dist(x, y));
  });
  return max;
}

console.log(task1());
console.log(task2());
