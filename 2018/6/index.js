const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((a) => a.split(", ").map((b) => Number(b)));

let minX = Infinity;
let maxX = -Infinity;
let minY = Infinity;
let maxY = -Infinity;

input.forEach(([x, y]) => {
  minX = Math.min(minX, x);
  minY = Math.min(minY, y);
  maxX = Math.max(maxX, x);
  maxY = Math.max(maxY, y);
});

function closest(x, y) {
  let min = Infinity;
  let minIndex;
  input.forEach(([xx, yy], i) => {
    const d = Math.abs(x - xx) + Math.abs(y - yy);
    if (d < min) {
      min = d;
      minIndex = i;
    }
  });
  return minIndex;
}

function task1() {
  const infinite = new Set();
  const counts = [];
  for (let x = minX; x <= maxX; x++) {
    for (let y = minY; y <= maxY; y++) {
      const c = closest(x, y);
      if (x === minX || x === maxX || y === minY || y === maxY) {
        infinite.add(c);
      }
      if (counts[c] === undefined) counts[c] = 0;
      counts[c]++;
    }
  }

  let max = 0;
  counts.forEach((c, i) => {
    if (infinite.has(i)) return;
    max = Math.max(max, c);
  });

  return max;
}

function distSum(x, y) {
  return input.reduce((a, [xx, yy]) => a + Math.abs(x - xx) + Math.abs(y - yy), 0);
}

function task2() {
  let c = 0;
  for (let x = minX; x <= maxX; x++) {
    for (let y = minY; y <= maxY; y++) {
      const d = distSum(x, y);
      if (x === minX || x === maxX || y === minY || y === maxY) {
        if (d < 10000) throw new Error(x + " " + y);
      }
      if (d < 10000) c++;
    }
  }

  return c;
}

console.log(task1());
console.log(task2());
