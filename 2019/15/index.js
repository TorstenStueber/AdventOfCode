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

function goto(path) {
  let state = { pc: 0n, ram: input, relativeBase: 0n, input: path, output: [] };
  let nextState;
  do {
    nextState = computerStep(state);
    if (nextState === undefined) break;
    if (nextState.output.length === path.length) {
      return nextState.output[path.length - 1];
    }
    state = nextState;
  } while (true);
}

function step(state, dir, x, y) {
  const newX = dir === 3n ? x - 1 : dir === 4n ? x + 1 : x;
  const newY = dir === 1n ? y - 1 : dir === 2n ? y + 1 : y;

  state.input = [dir];
  do {
    nextState = computerStep(state);
    if (nextState === undefined) break;
    if (nextState.output.length === 1) {
      const result = nextState.output[0];
      nextState.output = [];
      return { state: nextState, result, x: result === 0n ? x : newX, y: result === 0n ? y : newY };
    }
    state = nextState;
  } while (true);
}

function dfs(startPath, findOxygen) {
  let state = { pc: 0n, ram: input, relativeBase: 0n, input: [], output: [] };
  let x = 0;
  let y = 0;

  while (startPath.length) {
    const next = step(state, startPath.shift(), x, y);
    state = next.state;
    x = next.x;
    y = next.y;
  }

  const fields = { [`${x};${y}`]: { d: 0, p: [] } };

  function iterateDir(currentState, x, y, dir) {
    const { d, p } = fields[`${x};${y}`];
    const next = step(currentState, dir, x, y);
    if (next.result !== 0n) {
      const nextKey = `${next.x};${next.y}`;
      if (fields[nextKey] === undefined || fields[nextKey].d > d + 1) {
        fields[nextKey] = { d: d + 1, p: [...p, dir] };
        if (next.result === 2n && findOxygen) {
          throw fields[nextKey];
        }
        iterate(next.state, next.x, next.y);
      }
    }
  }

  function iterate(currentState, x, y) {
    iterateDir(currentState, x, y, 1n);
    iterateDir(currentState, x, y, 2n);
    iterateDir(currentState, x, y, 3n);
    iterateDir(currentState, x, y, 4n);
  }

  iterate(state, x, y);
  return fields;
}

function task1() {
  try {
    dfs([], true);
  } catch (result) {
    return result.d;
  }
}

function task2() {
  try {
    dfs([], true);
  } catch (result) {
    const startPath = result.p;
    const fields = dfs(startPath);
    return Object.keys(fields).reduce((acc, k) => Math.max(acc, fields[k].d), 0);
  }
}

console.log(task1());
console.log(task2());
