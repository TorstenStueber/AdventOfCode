const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("")
  .map((a) => Number(a));

function task1() {
  return input.reduce((a, b, i) => a + (b === input[(i + 1) % input.length] ? b : 0), 0);
}

function task2() {
  return input.reduce((a, b, i) => a + (b === input[(i + input.length / 2) % input.length] ? b : 0), 0);
}

console.log(task1());
console.log(task2());
