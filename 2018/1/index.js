const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function task1() {
  return input.reduce((a, b) => a + Number(b), 0);
}

function task2() {
  let freq = 0;
  let fs = new Set();
  while (true) {
    for (line of input) {
      if (fs.has(freq)) return freq;
      fs.add(freq);
      freq += Number(line);
    }
  }
}

console.log(task1());
console.log(task2());
