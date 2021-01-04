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

function camera() {
  const view = [];
  let line = "";
  let state = { pc: 0n, ram: input, relativeBase: 0n, input: [], output: [] };
  let nextState;
  do {
    nextState = computerStep(state);
    if (nextState === undefined) break;
    if (nextState.output.length === 1) {
      if (nextState.output[0] === 10n) {
        view.push(line);
        line = "";
      } else {
        line += String.fromCharCode(Number(nextState.output[0]));
      }
      nextState.output = [];
    }
    state = nextState;
  } while (true);

  return view;
}

function task1() {
  const view = camera();

  let count = 0;

  view.forEach((line, y) => {
    line.split("").forEach((symbol, x) => {
      if (
        0 < y &&
        y < view.length - 1 &&
        0 < x &&
        x < view.length - 1 &&
        view[y - 1][x] === "#" &&
        view[y + 1][x] === "#" &&
        view[y][x] === "#" &&
        view[y][x - 1] === "#" &&
        view[y][x + 1] === "#"
      ) {
        count += x * y;
      }
    });
  });

  return count;
}

function step(view, dir, x, y) {
  const nextX = dir === ">" ? x + 1 : dir === "<" ? x - 1 : x;
  const nextY = dir === "^" ? y - 1 : dir === "v" ? y + 1 : y;

  if (0 <= nextY && nextY < view.length && 0 <= nextX && nextX < view[nextY].length && view[nextY][nextX] === "#") {
    return { x: nextX, y: nextY };
  }
  return;
}

function determinePath() {
  const view = camera();
  let x;
  let y;
  view.find((line, yy) => {
    const found = line.split("").findIndex((c) => ["^", "v", "<", ">"].indexOf(c) >= 0);
    if (found >= 0) {
      x = found;
      y = yy;
    }
  });

  let dir = view[y][x];

  const result = [];
  while (true) {
    let leftDir;
    let rightDir;
    switch (dir) {
      case "^":
        leftDir = "<";
        rightDir = ">";
        break;
      case "v":
        leftDir = ">";
        rightDir = "<";
        break;
      case "<":
        leftDir = "v";
        rightDir = "^";
        break;
      case ">":
        leftDir = "^";
        rightDir = "v";
        break;
    }

    const nextPosL = step(view, leftDir, x, y);
    const nextPosR = step(view, rightDir, x, y);

    if (nextPosL) {
      dir = leftDir;
      result.push("L");
    } else if (nextPosR) {
      dir = rightDir;
      result.push("R");
    } else break;

    let steps = 0;
    do {
      const nextPos = step(view, dir, x, y);
      if (nextPos !== undefined) {
        steps++;
        x = nextPos.x;
        y = nextPos.y;
      } else {
        break;
      }
    } while (true);

    result.push(steps);
  }

  return result;
}

const program = `A,B,A,B,C,A,B,C,A,C
R,6,L,10,R,8
R,8,R,12,L,8,L,8
L,10,R,6,R,6,L,8
n
`;

function task2() {
  const view = [];
  const ram = [...input];
  ram[0] = 2n;
  let state = {
    pc: 0n,
    ram,
    relativeBase: 0n,
    input: program.split("").map((a) => BigInt(a.charCodeAt(0))),
    output: [],
  };

  let nextState;
  let line = "";
  let lastSymbol;
  do {
    nextState = computerStep(state);
    if (nextState === undefined) break;
    if (nextState.output.length === 1) {
      if (nextState.output[0] === 10n) {
        //console.log(line);
        line = "";
      } else {
        line += String.fromCharCode(Number(nextState.output[0]));
        lastSymbol = nextState.output[0];
      }
      nextState.output = [];
    }
    state = nextState;
  } while (true);

  return Number(lastSymbol);
}

console.log(task1());
console.log(task2());
