const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split(",")
  .map((a) => BigInt(a));

function computerStep({ pc, ram, relativeBase, input, output }) {
  const read = (pos) =>
    (ram[pc] % 10n ** (pos + 2n) > 2n * 10n ** (pos + 1n)
      ? ram[ram[pc + pos] + relativeBase]
      : ram[pc] % 10n ** (pos + 2n) > 10n ** (pos + 1n)
      ? ram[pc + pos]
      : ram[ram[pc + pos]]) ?? 0n;

  const write = (pos, value) => {
    const newRam = [...ram];
    if (newRam[pc] % 10n ** (pos + 2n) > 2n * 10n ** (pos + 1n)) {
      newRam[newRam[pc + pos] + relativeBase] = value;
    } else {
      newRam[newRam[pc + pos]] = value;
    }
    return newRam;
  };

  switch (ram[pc] % 100n) {
    case 1n: {
      return { pc: pc + 4n, ram: write(3n, read(1n) + read(2n)), relativeBase, input, output };
    }
    case 2n: {
      return { pc: pc + 4n, ram: write(3n, read(1n) * read(2n)), relativeBase, input, output };
    }
    case 3n: {
      if (input.length === 0) {
        // wait
        return { pc, ram, relativeBase, input, output };
      }
      return { pc: pc + 2n, ram: write(1n, input[0]), relativeBase, input: input.slice(1), output };
    }
    case 4n: {
      const newOuput = [...output, read(1n)];
      return { pc: pc + 2n, ram, relativeBase, input, output: newOuput };
    }
    case 5n: {
      return { pc: read(1n) !== 0n ? read(2n) : pc + 3n, ram, relativeBase, input, output };
    }
    case 6n: {
      return { pc: read(1n) === 0n ? read(2n) : pc + 3n, ram, relativeBase, input, output };
    }
    case 7n: {
      return { pc: pc + 4n, ram: write(3n, read(1n) < read(2n) ? 1n : 0n), relativeBase, input, output };
    }
    case 8n: {
      return { pc: pc + 4n, ram: write(3n, read(1n) === read(2n) ? 1n : 0n), relativeBase, input, output };
    }
    case 9n: {
      return { pc: pc + 2n, ram, relativeBase: relativeBase + read(1n), input, output };
    }
    case 99n:
      return undefined;
    default:
      throw new Error(`unknown opcode: ${ram} ... ${pc}... ${ram[pc]}`);
  }
}

function runProgam(program, mode) {
  program += `\n${mode}\n`;
  const springScript = program.split("").map((a) => BigInt(a.charCodeAt(0)));
  let state = { pc: 0n, ram: input, relativeBase: 0n, input: springScript, output: [] };
  let nextState;
  do {
    try {
      nextState = computerStep(state);
    } catch {
      console.log(state.output);
      break;
    }
    if (nextState === undefined) break;
    state = nextState;
  } while (true);

  if (state.output[state.output.length - 1] > 128n) {
    return state.output[state.output.length - 1];
  }
  console.log(state.output.map((a) => String.fromCharCode(Number(a))).join(""));
}

// d && !(a & b & c)
program1 = `
NOT B T
NOT T T
AND A T
AND C T
NOT T J
AND D J
`;

function task1() {
  return Number(runProgam(program1.trim(), "WALK"));
}

// d && !(a & b & c) && (e || h)
program2 = `
NOT B T
NOT T T
AND A T
AND C T
NOT T J
AND D J
NOT E T
NOT T T
OR H T
AND T J
`;

function task2() {
  return Number(runProgam(program2.trim(), "RUN"));
}

console.log(task1());
console.log(task2());
