const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((line) => line.split(": ").map((a) => Number(a)));

function task1() {
  return input.reduce((a, [d, r]) => a + (d % ((r - 1) * 2) === 0 ? d * r : 0), 0);
}

function task2() {
  for (let e = 0; true; e++) {
    if (input.every(([d, r]) => (d + e) % ((r - 1) * 2) !== 0)) return e;
  }
}

console.log(task1());
console.log(task2());
