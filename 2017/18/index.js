const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function exec(state, snd, rcv, isVersion2) {
  let inst = input[state.pc];
  const ops = inst
    .slice(4)
    .split(" ")
    .map((a) => (/-?\d+/.exec(a) ? Number(a) : state.regs[a] ?? 0));

  const leftOps = inst.slice(4).split(" ");

  switch (inst.slice(0, 3)) {
    case "snd":
      snd.push(ops[0]);
      state.pc++;
      break;
    case "set":
      state.regs[leftOps[0]] = ops[1];
      state.pc++;
      break;
    case "add":
      state.regs[leftOps[0]] = ops[0] + ops[1];
      state.pc++;
      break;
    case "mul":
      state.regs[leftOps[0]] = ops[0] * ops[1];
      state.pc++;
      break;
    case "mod":
      state.regs[leftOps[0]] = ops[0] % ops[1];
      state.pc++;
      break;
    case "rcv":
      if (isVersion2) {
        if (rcv.length === 0) return;
        state.regs[leftOps[0]] = rcv.shift();
        state.pc++;
      } else {
        if (ops[0] !== 0) rcv.push(snd[snd.length - 1]);
        state.pc++;
      }
      break;
    case "jgz":
      if (ops[0] > 0) {
        state.pc += ops[1];
      } else {
        state.pc++;
      }
      break;
  }
}

function task1() {
  const state = { pc: 0, regs: {} };
  const output = [];
  const rcv = [];
  while (rcv.length === 0) exec(state, output, rcv);

  return rcv[0];
}

function task2() {
  const state1 = { pc: 0, regs: { p: 0 } };
  const state2 = { pc: 0, regs: { p: 1 } };
  const queue1to2 = [];
  const queue2to1 = [];

  let c = 0;
  while (state1.pc < input.length && state2.pc < input.length) {
    let pc1 = state1.pc;
    let pc2 = state2.pc;
    exec(state1, queue1to2, queue2to1, true);
    let n = queue2to1.length;
    exec(state2, queue2to1, queue1to2, true);
    if (queue2to1.length !== n) c++;

    if (pc1 === state1.pc && pc2 === state2.pc) break;
  }

  return c;
}

console.log(task1());
console.log(task2());
