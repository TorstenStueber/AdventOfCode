const { readFileSync } = require("fs");
const { join } = require("path");

const rawInput = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");
const input = rawInput.slice(1);
const binding = Number(/#ip (\d)/.exec(rawInput[0])[1]);

function runInst(regs, code) {
  const inst = code[regs[binding]];
  const [op, s1, s2, s3] = inst.split(" ");
  const [i1, i2, i3] = [s1, s2, s3].map((a) => Number(a));
  switch (op) {
    case "addr":
      return { ...regs, [i3]: regs[i1] + regs[i2] };
    case "addi":
      return { ...regs, [i3]: regs[i1] + i2 };
    case "mulr":
      return { ...regs, [i3]: regs[i1] * regs[i2] };
    case "muli":
      return { ...regs, [i3]: regs[i1] * i2 };
    case "banr":
      return { ...regs, [i3]: regs[i1] & regs[i2] };
    case "bani":
      return { ...regs, [i3]: regs[i1] & i2 };
    case "borr":
      return { ...regs, [i3]: regs[i1] | regs[i2] };
    case "bori":
      return { ...regs, [i3]: regs[i1] | i2 };
    case "setr":
      return { ...regs, [i3]: regs[i1] };
    case "seti":
      return { ...regs, [i3]: i1 };
    case "gtir":
      return { ...regs, [i3]: i1 > regs[i2] ? 1 : 0 };
    case "gtri":
      return { ...regs, [i3]: regs[i1] > i2 ? 1 : 0 };
    case "gtrr":
      return { ...regs, [i3]: regs[i1] > regs[i2] ? 1 : 0 };
    case "eqir":
      return { ...regs, [i3]: i1 === regs[i2] ? 1 : 0 };
    case "eqri":
      return { ...regs, [i3]: regs[i1] === i2 ? 1 : 0 };
    case "eqrr":
      return { ...regs, [i3]: regs[i1] === regs[i2] ? 1 : 0 };
    case "nop":
      return regs;
    case "factSum": {
      let c = 0;
      for (i = 1; i <= regs[3]; i++) if (regs[3] % i === 0) c += i;
      return { ...regs, [0]: c };
    }
    default:
      throw Error("undefined instruction");
  }
}

function task1() {
  let regs = [0, 0, 0, 0, 0, 0];
  while (regs[binding] < input.length) {
    regs = runInst(regs, input);
    regs[binding]++;
  }

  return regs[0];
}

const factorSumCode = `seti 1 4 5
seti 1 4 2
mulr 5 2 4
eqrr 4 3 4
addr 4 1 1
addi 1 1 1
addr 5 0 0
addi 2 1 2
gtrr 2 3 4
addr 1 4 1
seti 2 6 1
addi 5 1 5
gtrr 5 3 4
addr 4 1 1
seti 1 7 1`;

function task2() {
  let regs = [1, 0, 0, 0, 0, 0];

  const code = [...input];
  if (input.slice(1, 1 + factorSumCode.split("\n").length).join("\n") === factorSumCode) {
    code[1] = "factSum 0 0 0";
    for (i = 1; i < factorSumCode.split("\n").length; i++) {
      code[1 + i] = "nop 0 0 0";
    }
  }

  while (regs[binding] < input.length) {
    regs = runInst(regs, code);
    regs[binding]++;
  }

  return regs[0];
}

console.log(task1());
console.log(task2());
