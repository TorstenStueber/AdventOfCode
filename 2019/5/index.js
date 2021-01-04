const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split(",")
  .map((a) => parseInt(a, 10));

function computerStep({ pc, ram, input, output }) {
  const read = (pos) => (ram[pc] % 10 ** (pos + 2) > 10 ** (pos + 1) ? ram[pc + pos] : ram[ram[pc + pos]]);
  switch (ram[pc] % 100) {
    case 1: {
      const newRam = [...ram];
      newRam[newRam[pc + 3]] = read(1) + read(2);
      return { pc: pc + 4, ram: newRam, input, output };
    }
    case 2: {
      const newRam = [...ram];
      newRam[newRam[pc + 3]] = read(1) * read(2);
      return { pc: pc + 4, ram: newRam, input, output };
    }
    case 3: {
      const newRam = [...ram];
      newRam[newRam[pc + 1]] = input[0];
      return { pc: pc + 2, ram: newRam, input: input.slice(1), output };
    }
    case 4: {
      const newOuput = [...output, read(1)];
      return { pc: pc + 2, ram, input, output: newOuput };
    }
    case 5: {
      return { pc: read(1) !== 0 ? read(2) : pc + 3, ram, input, output };
    }
    case 6: {
      return { pc: read(1) === 0 ? read(2) : pc + 3, ram, input, output };
    }
    case 7: {
      const newRam = [...ram];
      newRam[newRam[pc + 3]] = read(1) < read(2) ? 1 : 0;
      return { pc: pc + 4, ram: newRam, input, output };
    }
    case 8: {
      const newRam = [...ram];
      newRam[newRam[pc + 3]] = read(1) === read(2) ? 1 : 0;
      return { pc: pc + 4, ram: newRam, input, output };
    }
    case 99:
      return undefined;
    default:
      throw new Error(`unknown opcode: ${ram}, ${pc}, ${ram[pc]}`);
  }
}

function iterateRam(ram, input) {
  let state = { pc: 0, ram, input, output: [] };
  let nextState;
  do {
    nextState = computerStep(state);
    if (nextState === undefined) return state;
    state = nextState;
  } while (true);
}

function task1() {
  const { output } = iterateRam(input, [1]);
  return output;
}

function task2() {
  const { output } = iterateRam(input, [5]);
  return output;
}

console.log(task1());
console.log(task2());
