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

function task1() {
  let states = [];
  for (let i = 0; i < 50; i++) {
    states.push({ pc: 0n, ram: input, relativeBase: 0n, input: [BigInt(i)], output: [] });
  }

  do {
    for (let i = 0; i < 50; i++) {
      const state = states[i];
      if (state.input.length === 0) state.input.push(-1n);
      let nextState = computerStep(state);
      if (nextState === undefined) break;
      states[i] = nextState;
      if (nextState.output.length === 3) {
        if (nextState.output[0] === 255n) {
          return Number(nextState.output[2]);
        }
        states[Number(nextState.output[0])].input.push(nextState.output[1]);
        states[Number(nextState.output[0])].input.push(nextState.output[2]);
        nextState.output = [];
      }
    }
  } while (true);
}

function task2() {
  let natX;
  let natY;
  let lastNatDeliveredY;
  const activityLog = [];

  let states = [];
  for (let i = 0; i < 50; i++) {
    states.push({ pc: 0n, ram: input, relativeBase: 0n, input: [BigInt(i)], output: [] });
    activityLog.push([]);
  }

  do {
    let hasQueuedMessage = false;
    for (let i = 0; i < 50; i++) {
      if (states[i].input.length === 0) states[i].input.push(-1n);
      if (
        states[i].input.length !== 1 ||
        states[i].input[0] !== -1n ||
        activityLog[i].length < 2 ||
        activityLog[i][0] !== "readEmpty" ||
        activityLog[i][1] !== "readEmpty"
      )
        hasQueuedMessage = true;
    }

    if (!hasQueuedMessage && natX !== undefined && natY !== undefined) {
      states[0].input.push(natX);
      states[0].input.push(natY);
      if (lastNatDeliveredY === natY) return Number(natY);
      lastNatDeliveredY = natY;
      for (let i = 0; i < 50; i++) {
        activityLog[i] = [];
      }
    }

    for (let i = 0; i < 50; i++) {
      const state = states[i];
      let nextState = computerStep(state);
      if (nextState === undefined) break;
      if (state.input.length === 1 && state.input[0] === -1n && nextState.input.length === 0) {
        activityLog[i].unshift("readEmpty");
      }
      states[i] = nextState;
      if (nextState.output.length === 3) {
        activityLog[i].unshift("sendMessage");
        if (nextState.output[0] === 255n) {
          natX = nextState.output[1];
          natY = nextState.output[2];
        } else {
          states[Number(nextState.output[0])].input.push(nextState.output[1]);
          states[Number(nextState.output[0])].input.push(nextState.output[2]);
        }
        nextState.output = [];
      }
    }
  } while (true);
}

console.log(task1());
console.log(task2());
