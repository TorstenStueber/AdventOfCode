const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n\n");

const row = input[0]
  .slice("initial state: ".length)
  .split("")
  .reduce((a, b, i) => {
    if (b === "#") a.add(String(i));
    return a;
  }, new Set());

const rules = {};
input[1].split("\n").forEach((line) => {
  const p = line.split(" => ");
  rules[p[0]] = p[1];
});

function step(state) {
  let min = Infinity;
  let max = -Infinity;

  Array.from(state).forEach((a) => {
    min = Math.min(min, Number(a));
    max = Math.max(max, Number(a));
  });

  const nextState = new Set();
  for (let i = min - 2; i <= max + 2; i++) {
    let string = "";
    for (let k = -2; k <= 2; k++) {
      string += state.has(String(i + k)) ? "#" : ".";
    }
    if (rules[string] === "#") nextState.add(String(i));
  }

  return nextState;
}

function task1() {
  let state = row;
  for (let i = 0; i < 20; i++) {
    state = step(state);
  }

  return Array.from(state).reduce((a, b) => a + Number(b), 0);
}

function task2() {
  const limit = 50000000000;
  let state = row;
  for (let i = 0; i < limit; i++) {
    const prevState = state;
    state = step(state);
    if (prevState.size === state.size) {
      if ((state.size = 0)) return 0;
      const a1 = Array.from(prevState)
        .map((a) => Number(a))
        .sort();
      const a2 = Array.from(state)
        .map((a) => Number(a))
        .sort();
      if (a1.every((a, i) => a2[i] === a + a2[0] - a1[0])) {
        const current = Array.from(state).reduce((a, b) => a + Number(b), 0);
        return current + (limit - 1 - i) * state.size * (a2[0] - a1[0]);
      }
    }
  }

  return Array.from(state).reduce((a, b) => a + Number(b), 0);
}

console.log(task1());
console.log(task2());
