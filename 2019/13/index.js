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

function paint() {
  const tiles = {};
  let state = { pc: 0n, ram: input, relativeBase: 0n, input: [], output: [] };
  let nextState;
  do {
    nextState = computerStep(state);
    if (nextState === undefined) return tiles;
    if (nextState.output.length === 3) {
      tiles[`${nextState.output[0]};${nextState.output[1]}`] = nextState.output[2];
      nextState.output = [];
    }
    state = nextState;
  } while (true);
}

function task1() {
  const tiles = {};
  let state = { pc: 0n, ram: input, relativeBase: 0n, input: [], output: [] };
  let nextState;
  do {
    nextState = computerStep(state);
    if (nextState === undefined) break;
    if (nextState.output.length === 3) {
      tiles[`${nextState.output[0]};${nextState.output[1]}`] = nextState.output[2];
      nextState.output = [];
    }
    state = nextState;
  } while (true);

  return Object.keys(tiles).filter((key) => tiles[key] === 2n).length;
}

function task2() {
  const ram = [...input];
  ram[0] = 2n;

  const tiles = [];
  let score;
  let state = { pc: 0n, ram, relativeBase: 0n, input: [], output: [] };
  let nextState;
  let ballX;
  let paddleX;
  do {
    nextState = computerStep(state);
    if (nextState === undefined) break;
    if (nextState.output.length === 3) {
      if (`${nextState.output[0]};${nextState.output[1]}` === "-1;0") {
        score = nextState.output[2];
        console.log(score);
        console.log(tiles.map((line) => line.join("")).join("\n"));
      } else {
        if (!tiles[nextState.output[1]]) tiles[nextState.output[1]] = [];
        let symbol;
        switch (nextState.output[2]) {
          case 0n:
            symbol = "⬜️";
            break;
          case 1n:
            symbol = "🧱";
            break;
          case 2n:
            symbol = "🎁";
            break;
          case 3n:
            symbol = "➖";
            paddleX = nextState.output[0];
            break;
          case 4n:
            symbol = "⚽️";
            ballX = nextState.output[0];
            nextState.input = [ballX > paddleX ? 1n : ballX < paddleX ? -1n : 0n];
            break;
        }
        tiles[nextState.output[1]][nextState.output[0]] = symbol;
      }
      nextState.output = [];
    }
    state = nextState;
  } while (true);

  return Object.keys(tiles).filter((key) => tiles[key] === 2n).length;
}

console.log(task1());
console.log(task2());
