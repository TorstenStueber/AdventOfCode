const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((a) => a.split(/\s+/).map((b) => Number(b)));

function task1() {
  return input.reduce((a, b) => {
    let max = 0;
    let min = Infinity;
    b.forEach((s) => {
      max = Math.max(max, s);
      min = Math.min(min, s);
    });
    return a + max - min;
  }, 0);
}

function task2() {
  return input.reduce((a, b) => {
    let r;
    for (let i = 0; i < b.length; i++) {
      for (let j = i + 1; j < b.length; j++) {
        if (b[i] % b[j] === 0) {
          r = b[i] / b[j];
        } else if (b[j] % b[i] === 0) {
          r = b[j] / b[i];
        }
        if (r !== undefined) break;
      }
      if (r !== undefined) break;
    }
    return a + r;
  }, 0);
}

console.log(task1());
console.log(task2());
