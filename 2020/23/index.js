const { strict } = require("assert");
const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("")
  .map((a) => Number(a));

function task1() {
  let state = [...input];
  const max = input.reduce((a, b) => Math.max(a, b), 0);
  for (let i = 0; i < 100; i++) {
    let j = state[0] - 1;
    while (state.slice(4).indexOf(j) === -1) {
      j--;
      if (j < 0) j = max;
    }
    const k = state.slice(4).indexOf(j) + 4;
    state = [...state.slice(4, k + 1), ...state.slice(1, 4), ...state.slice(k + 1), state[0]];
  }

  const k = state.indexOf(1);
  return [...state.slice(k + 1), ...state.slice(0, k)].join("");
}

function task2() {
  let state = [...input];
  for (let i = state.length; i < 1000000; i++) state.push(i + 1);

  const list = [];

  state.forEach((a, i) => {
    list[a - 1] = { next: state[i < state.length - 1 ? i + 1 : 0] - 1 };
  });

  let pos = input[0] - 1;
  for (let i = 0; i < 10000000; i++) {
    const next = list[pos].next;
    const nnext = list[next].next;
    const nnnext = list[nnext].next;
    const nnnnext = list[nnnext].next;

    let cPos = pos;
    do {
      cPos--;
      if (cPos < 0) cPos = list.length - 1;
    } while (cPos === pos || cPos === next || cPos === nnext || cPos === nnnext);

    list[pos].next = nnnnext;
    list[nnnext].next = list[cPos].next;
    list[cPos].next = next;
    pos = nnnnext;
  }

  return (list[0].next + 1) * (list[list[0].next].next + 1);
}

console.log(task1());
console.log(task2());
