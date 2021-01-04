const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((a) => parseInt(a, 10));

function task1() {
  return input.reduce((acc, line) => acc + Math.floor(line / 3) - 2, 0);
}

function fuelOffuel(fuel) {
  if (fuel <= 0) return 0;
  return fuel + fuelOffuel(Math.floor(fuel / 3) - 2);
}

function task2() {
  return input.reduce(
    (acc, line) => acc + fuelOffuel(Math.floor(line / 3) - 2),
    0
  );
}

console.log(task1());
console.log(task2());
