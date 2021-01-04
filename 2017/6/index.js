const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split(/\s+/)
  .map((a) => Number(a));

function task1() {
  let state = [...input];
  const states = new Set();
  let c = 0;
  while (true) {
    const key = state.join(",");
    if (states.has(key)) return c;
    states.add(key);

    let max = -Infinity;
    let maxIndex;
    state.forEach((a, i) => {
      if (a > max) {
        max = a;
        maxIndex = i;
      }
    });

    state[maxIndex] = 0;
    while (max > 0) {
      maxIndex = (maxIndex + 1) % state.length;
      state[maxIndex]++;
      max--;
    }
    c++;
  }
}

function task2() {
  let state = [...input];
  const states = {};
  let c = 0;
  while (true) {
    const key = state.join(",");
    if (states[key] !== undefined) return c - states[key];
    states[key] = c;

    let max = -Infinity;
    let maxIndex;
    state.forEach((a, i) => {
      if (a > max) {
        max = a;
        maxIndex = i;
      }
    });

    state[maxIndex] = 0;
    while (max > 0) {
      maxIndex = (maxIndex + 1) % state.length;
      state[maxIndex]++;
      max--;
    }
    c++;
  }
}

console.log(task1());
console.log(task2());
