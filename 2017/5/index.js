const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((a) => Number(a));

function task1() {
  let pc = 0;
  let state = [...input];
  let c = 0;
  while (pc < state.length) {
    c++;
    state[pc]++;
    pc += state[pc] - 1;
  }
  return c;
}

function task2() {
  let pc = 0;
  let state = [...input];
  let c = 0;
  while (pc < state.length) {
    let oldPc = pc;
    c++;
    pc += state[pc];
    if (state[oldPc] >= 3) {
      state[oldPc]--;
    } else {
      state[oldPc]++;
    }
  }
  return c;
}

console.log(task1());
console.log(task2());
