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
      throw new Error(`unknown opcode: ${ram}, ${pc}, ${ram[pc]}`);
  }
}

function paint(startColor) {
  const panels = {};
  let x = 0;
  let y = 0;
  let state = { pc: 0n, ram: input, relativeBase: 0n, input: [startColor], output: [] };
  let nextState;
  let dir = 0;
  do {
    nextState = computerStep(state);
    if (nextState === undefined) return panels;
    if (nextState.output.length === 2) {
      panels[`${x};${y}`] = nextState.output[0];

      if (nextState.output[1] === 0n) {
        dir--;
      } else {
        dir++;
      }
      if (dir < 0) dir += 4;

      switch (dir % 4) {
        case 0:
          y--;
          break;
        case 1:
          x++;
          break;
        case 2:
          y++;
          break;
        case 3:
          x--;
          break;
      }

      nextState.output = [];
      nextState.input = [panels[`${x};${y}`] ?? 0n];
    }
    state = nextState;
  } while (true);
}

function task1() {
  const panels = paint(0n);
  return Object.keys(panels).length;
}

function task2() {
  const panels = paint(1n);
  let minX = Infinity,
    maxX = -Infinity,
    minY = Infinity,
    maxY = -Infinity;
  Object.keys(panels).forEach((key) => {
    const [x, y] = key.split(";");
    minX = Math.min(minX, x);
    maxX = Math.max(maxX, x);
    minY = Math.min(minY, y);
    maxY = Math.max(maxY, y);
  });

  result = [];
  for (let y = minY; y <= maxY; y++) {
    let line = "";
    for (let x = minX; x <= maxX; x++) {
      line += (panels[`${x};${y}`] ?? 0n) === 0n ? " " : "#";
    }
    result.push(line);
  }
  return result.join("\n");
}

console.log(task1());
console.log(task2());
