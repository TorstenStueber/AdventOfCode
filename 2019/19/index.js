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

function runAt(x, y) {
  let state = { pc: 0n, ram: input, relativeBase: 0n, input: [BigInt(x), BigInt(y)], output: [] };
  let nextState;
  do {
    nextState = computerStep(state);
    if (nextState === undefined) break;
    if (nextState.output.length === 1) {
      return Number(nextState.output[0]);
    }
    state = nextState;
  } while (true);
}

const minX = [0];
const maxX = [0];
let maxY = 0;

const map = [];

function getPos(x, y) {
  const key = `${x};${y}`;
  if (map[key] === undefined) {
    let min = minX[maxY];
    let max = maxX[maxY] + 1;
    while (maxY < y) {
      maxY++;
      tries = 20;
      while (runAt(min, maxY) === 0 && tries-- > 0) min++;
      if (tries <= 0) {
        min = 0;
        max = 0;
      } else {
        max = Math.max(max, min);
        while (runAt(max, maxY) === 1) max++;
      }
      minX[maxY] = min;
      maxX[maxY] = max - 1;
    }
    map[key] = minX[y] <= x && x <= maxX[y] ? 1 : 0;
  }
  return map[key];
}

function task1() {
  let c = 0;
  for (let y = 0; y < 50; y++) {
    for (let x = 0; x < 50; x++) {
      if (getPos(x, y) === 1) c++;
    }
  }
  return c;
}

function task2() {
  for (let d = 0; true; d++) {
    for (let x = 0; x <= d; x++) {
      const y = d - x;
      if (y > maxY) getPos(0, y);
      if (x < minX[y] || x > maxX[y] - 99) continue;
      let stop = false;

      for (let xx = 0; xx < 100; xx++) {
        for (let yy = 0; yy < 100; yy++) {
          if (getPos(x + xx, y + yy) === 0) {
            stop = true;
            break;
          }
        }
        if (stop) break;
      }
      if (!stop) return x * 10000 + y;
    }
  }
}

console.log(task1());
console.log(task2());
