const { readFileSync } = require("fs");
const { join } = require("path");

const stringInput = readFileSync(join(__dirname, "input.txt"), "utf-8").trim();

const input = Number(stringInput);

function task1() {
  let state = [3, 7];
  let e1 = 0;
  let e2 = 1;
  while (state.length < input + 10) {
    const s = state[e1] + state[e2];
    if (s >= 10) {
      state.push(1);
      state.push(s - 10);
    } else {
      state.push(s);
    }
    e1 = (e1 + state[e1] + 1) % state.length;
    e2 = (e2 + state[e2] + 1) % state.length;
  }

  return state.slice(input, input + 10).join("");
}

function task2() {
  let state = [3, 7];
  let e1 = 0;
  let e2 = 1;
  while (true) {
    const s = state[e1] + state[e2];
    if (s >= 10) {
      state.push(1);
      if (state.slice(-stringInput.length).join("") === stringInput) return state.length - stringInput.length;
      state.push(s - 10);
      if (state.slice(-stringInput.length).join("") === stringInput) return state.length - stringInput.length;
    } else {
      state.push(s);
      if (state.slice(-stringInput.length).join("") === stringInput) return state.length - stringInput.length;
    }
    e1 = (e1 + state[e1] + 1) % state.length;
    e2 = (e2 + state[e2] + 1) % state.length;
  }
}

console.log(task1());
console.log(task2());
