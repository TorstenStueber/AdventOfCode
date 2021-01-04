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
      if (input.length === 0) {
        // wait
        return { pc, ram, input, output };
      }
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

function iterateMachines(initialInputs) {
  const input1 = [Number(initialInputs[0]) + 5, 0];
  const input2 = [Number(initialInputs[1]) + 5];
  const input3 = [Number(initialInputs[2]) + 5];
  const input4 = [Number(initialInputs[3]) + 5];
  const input5 = [Number(initialInputs[4]) + 5];

  let state1 = { pc: 0, ram: input, input: input1, output: [] };
  let state2 = { pc: 0, ram: input, input: input2, output: [] };
  let state3 = { pc: 0, ram: input, input: input3, output: [] };
  let state4 = { pc: 0, ram: input, input: input4, output: [] };
  let state5 = { pc: 0, ram: input, input: input5, output: [] };

  let lastOutput = undefined;

  do {
    const nextState1 = computerStep(state1);
    if (nextState1 !== undefined) {
      state1 = nextState1;
      state2.input = [...state2.input, ...nextState1.output];
      nextState1.output = [];
    }

    const nextState2 = computerStep(state2);
    if (nextState2 !== undefined) {
      state2 = nextState2;
      state3.input = [...state3.input, ...nextState2.output];
      nextState2.output = [];
    }

    const nextState3 = computerStep(state3);
    if (nextState3 !== undefined) {
      state3 = nextState3;
      state4.input = [...state4.input, ...nextState3.output];
      nextState3.output = [];
    }

    const nextState4 = computerStep(state4);
    if (nextState4 !== undefined) {
      state4 = nextState4;
      state5.input = [...state5.input, ...nextState4.output];
      nextState4.output = [];
    }

    const nextState5 = computerStep(state5);
    if (nextState5 !== undefined) {
      state5 = nextState5;
      state1.input = [...state1.input, ...nextState5.output];
      if (nextState5.output.length) lastOutput = nextState5.output[0];
      nextState5.output = [];
    }

    if (!nextState1 && !nextState2 && !nextState3 && !nextState4 && !nextState5) return lastOutput;
  } while (true);
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
  let maxAmp = 0;
  for (let i = 0; i < 5 ** 5; i++) {
    const program = i.toString(5).padStart(5, "0");
    if (["0", "1", "2", "3", "4"].some((a) => program.indexOf(a) === -1)) continue;
    const { output: o1 } = iterateRam(input, [Number(program[0]), 0]);
    const { output: o2 } = iterateRam(input, [Number(program[1]), o1[0]]);
    const { output: o3 } = iterateRam(input, [Number(program[2]), o2[0]]);
    const { output: o4 } = iterateRam(input, [Number(program[3]), o3[0]]);
    const { output: o5 } = iterateRam(input, [Number(program[4]), o4[0]]);
    const amp = o5[0];
    if (amp > maxAmp) maxAmp = amp;
  }
  return maxAmp;
}

function task2() {
  let maxAmp = 0;
  for (let i = 0; i < 5 ** 5; i++) {
    const program = i.toString(5).padStart(5, "0");
    if (["0", "1", "2", "3", "4"].some((a) => program.indexOf(a) === -1)) continue;
    const amp = iterateMachines(program);
    if (amp > maxAmp) maxAmp = amp;
  }
  return maxAmp;
}

console.log(task1());
console.log(task2());
