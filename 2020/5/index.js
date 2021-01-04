const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function pos(input) {
  return (
    parseInt(input.slice(0, 7).replace(/F/g, 0).replace(/B/g, 1), 2) * 8 +
    parseInt(input.slice(7).replace(/L/g, 0).replace(/R/g, 1), 2)
  );
}

function task1() {
  return input.reduce((a, b) => Math.max(a, pos(b)), 0);
}

function task2() {
  const a = new Set();
  for (let i = 0; i < 128 * 8; i++) {
    a.add(i);
  }
  const poses = input.map((l) => pos(l)).sort();
  const found =
    poses.find((x, i) => {
      return i < poses.length - 1 && poses[i + 1] === x + 2;
    }) + 1;

  return found;
}

console.log(task1());
console.log(task2());
