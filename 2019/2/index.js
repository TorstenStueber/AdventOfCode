const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split(",")
  .map((a) => parseInt(a, 10));

function computerStep({ pc, ram }) {
  switch (ram[pc]) {
    case 1: {
      const newRam = [...ram];
      newRam[newRam[pc + 3]] = newRam[newRam[pc + 1]] + newRam[newRam[pc + 2]];
      return { pc: pc + 4, ram: newRam };
    }
    case 2: {
      const newRam = [...ram];
      newRam[newRam[pc + 3]] = newRam[newRam[pc + 1]] * newRam[newRam[pc + 2]];
      return { pc: pc + 4, ram: newRam };
    }
    case 99:
      return undefined;
    default:
      throw new Error("unknown opcode", ram, pc, ram[pc]);
  }
}

function iterateRam(ram) {
  let state = { pc: 0, ram };
  let nextState;
  do {
    nextState = computerStep(state);
    if (nextState === undefined) return state;
    state = nextState;
  } while (true);
}

function task1() {
  const i = [...input];
  i[1] = 12;
  i[2] = 2;
  const { ram } = iterateRam(i);
  return ram[0];
}

function runOnInput(noun, verb) {
  const i = [...input];
  i[1] = noun;
  i[2] = verb;
  const { ram } = iterateRam(i);
  return ram[0];
}

function task2() {
  for (let noun = 0; noun <= 99; noun++)
    for (let verb = 0; verb <= 99; verb++)
      if (runOnInput(noun, verb) === 19690720) return noun * 100 + verb;
}

console.log(task1());
console.log(task2());
