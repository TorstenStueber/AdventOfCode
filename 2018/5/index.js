const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("");

function resolve(state) {
  while (true) {
    let found = false;
    for (let i = 0; i < state.length - 1; i++) {
      if (state[i].toLowerCase() === state[i + 1].toLowerCase() && state[i] !== state[i + 1]) {
        found = true;
        state.splice(i, 2);
        break;
      }
    }
    if (!found) break;
  }

  return state;
}

const r = resolve([...input]);

function task1() {
  return r.length;
}

function task2() {
  const letters = new Set();
  r.forEach((a) => letters.add(a.toLowerCase()));

  return Array.from(letters).reduce(
    (a, b) => Math.min(a, resolve([...r].filter((x) => x.toLowerCase() !== b)).length),
    Infinity
  );
}

console.log(task1());
console.log(task2());
