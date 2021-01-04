const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((a) => parseInt(a, 10));

function task1() {
  for (line1 of input) {
    for (line2 of input) {
      if (line1 + line2 === 2020) return line1 * line2;
    }
  }
}

function task2() {
  for (line1 of input) {
    for (line2 of input) {
      for (line3 of input) {
        if (line1 + line2 + line3 === 2020) return line1 * line2 * line3;
      }
    }
  }
}

console.log(task1());
console.log(task2());
