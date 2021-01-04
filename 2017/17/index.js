const { readFileSync } = require("fs");
const { join } = require("path");

const input = Number(readFileSync(join(__dirname, "input.txt"), "utf-8").trim());

function task1() {
  const next = [0];
  let current = 0;
  for (let i = 1; i <= 2017; i++) {
    let n = input;
    while (n--) current = next[current];
    next.push(next[current]);
    next[current] = i;
    current = i;
  }

  return next[current];
}

function task2() {
  let current = 0;
  let l = 1;
  let lastI;
  for (let i = 1; i < 50000000; i++) {
    current = ((current + input) % l) + 1;
    l++;
    if (current === 1) {
      lastI = i;
    }
  }
  return lastI;
}

console.log(task1());
console.log(task2());
