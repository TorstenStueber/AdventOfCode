const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

const keys = [
  [1, 2, 3],
  [4, 5, 6],
  [7, 8, 9],
];

const dirs = { U: [0, -1], D: [0, 1], L: [-1, 0], R: [1, 0] };

function task1() {
  let x = 1;
  let y = 1;
  const result = [];

  input.forEach((line) => {
    line.split("").forEach((s) => {
      const [xx, yy] = dirs[s];
      if (x + xx < 0 || x + xx > 2 || y + yy < 0 || y + yy > 2) return;
      x += xx;
      y += yy;
    });
    result.push(keys[y][x]);
  });

  return result.join("");
}

const keys2 = [
  [undefined, undefined, 1],
  [undefined, 2, 3, 4],
  [5, 6, 7, 8, 9],
  [undefined, "A", "B", "C"],
  [undefined, undefined, "D"],
];

function task2() {
  let x = 1;
  let y = 1;
  const result = [];

  input.forEach((line) => {
    line.split("").forEach((s) => {
      const [xx, yy] = dirs[s];
      if (keys2[y + yy]?.[x + xx] === undefined) return;
      x += xx;
      y += yy;
    });
    result.push(keys2[y][x]);
  });

  return result.join("");
}

console.log(task1());
console.log(task2());
