const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((a) => a.split("-").map((b) => Number(b)));

function task1() {
  let min = 0;
  do {
    prevMin = min;
    input.forEach(([a, b]) => {
      if (a <= min && min <= b) min = b + 1;
    });
  } while (prevMin !== min);

  return min;
}

function task2() {
  let regions = [[0, 2 ** 32 - 1]];
  input.forEach(([a, b]) => {
    const newRegions = [];
    regions.forEach((reg) => {
      if (reg[0] < a) newRegions.push([reg[0], Math.min(reg[1], a - 1)]);
      if (reg[1] > b) newRegions.push([Math.max(reg[0], b + 1), reg[1]]);
    });
    regions = newRegions;
  });
  return regions.reduce((a, [b, c]) => a + b - c + 1, 0);
}

console.log(task1());
console.log(task2());
