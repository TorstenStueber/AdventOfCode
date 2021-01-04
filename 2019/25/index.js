const { readFileSync } = require("fs");
const { join } = require("path");

const readline = require("readline");

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

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

const collection = `south
south
south
take fixed point
south
take festive hat
west
west
take jam
south
take easter egg
north
east
east
north
west
take asterisk
east
north
west
north
north
take tambourine
south
south
east
north
west
south
take antenna
north
west
west
take space heater
west
`;

const items = ["asterisk", "antenna", "easter egg", "space heater", "jam", "tambourine", "festive hat", "fixed point"];

function task1() {
  let state = { pc: 0n, ram: input, relativeBase: 0n, input: [], output: [] };

  function runToQuestion(input) {
    input.split("").forEach((a) => state.input.push(BigInt(a.charCodeAt(0))));
    do {
      let nextState = computerStep(state);
      if (nextState === undefined) break;
      state = nextState;
      if (state.output.length && state.output[state.output.length - 1] === 10n) {
        const line = state.output.map((a) => String.fromCharCode(Number(a))).join("");
        console.log(line);
        state.output = [];
        if (line === "Command?\n" && state.input.length === 0) return;
      }
    } while (true);
  }

  rl.on("line", (input) => {
    runToQuestion(input + "\n");
  });

  let exploreEach = "";
  for (i = 0; i < 2 ** items.length; i++) {
    const dropItems = items.filter((_, j) => (i >> j) & (1 === 1));
    dropItems.forEach((item) => (exploreEach += `drop ${item}\n`));
    exploreEach += `west\n`;
    dropItems.forEach((item) => (exploreEach += `take ${item}\n`));
  }

  runToQuestion(collection + exploreEach);
}

console.log(task1());
