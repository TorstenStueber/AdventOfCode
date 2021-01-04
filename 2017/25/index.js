const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").split("\n\n");

function task1() {
  let state;
  let steps;
  let pos = 0;
  let tape = {};

  let table = {};

  input.forEach((s, i) => {
    const p = s.split("\n");
    if (i === 0) {
      const line1 = /Begin in state (\w+)\./.exec(p[0]);
      const line2 = /Perform a diagnostic checksum after (\d+) steps\./.exec(p[1]);

      state = line1[1];
      steps = Number(line2[1]);
    } else {
      const [_, state] = /In state (\w+):/.exec(p[0]);
      const subTable = {};
      for (let i = 1; i < p.length; i += 4) {
        const [_1, v] = /  If the current value is (\d):/.exec(p[i]);
        const [_2, nextV] = /    - Write the value (\d)\./.exec(p[i + 1]);
        const [_3, dir] = /    - Move one slot to the (left|right)\./.exec(p[i + 2]);
        const [_4, nextState] = /    - Continue with state (\w+)\./.exec(p[i + 3]);
        subTable[v] = { v: nextV, dir, s: nextState };
      }
      table[state] = subTable;
    }
  });

  while (steps--) {
    const { v, dir, s } = table[state][tape[`${pos}`] ?? "0"];
    tape[`${pos}`] = v;
    pos = dir === "right" ? pos + 1 : pos - 1;
    state = s;
  }

  return Object.keys(tape).reduce((a, b) => a + (tape[b] === "1" ? 1 : 0), 0);
}

console.log(task1());
