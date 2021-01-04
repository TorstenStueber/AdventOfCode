const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n\n\n");

const samples = input[0].split("\n\n").map((sample) => {
  const [line1, line2, line3] = sample.split("\n");
  return {
    before: /Before: \[(.*)\]/
      .exec(line1)[1]
      .split(", ")
      .map((a) => Number(a)),
    instr: line2.split(" ").map((a) => Number(a)),
    after: /After:  \[(.*)\]/
      .exec(line3)[1]
      .split(", ")
      .map((a) => Number(a)),
  };
});

const ops = [
  "addr",
  "addi",
  "mulr",
  "muli",
  "banr",
  "bani",
  "borr",
  "bori",
  "setr",
  "seti",
  "gtir",
  "gtri",
  "gtrr",
  "eqir",
  "eqri",
  "eqrr",
];

function runInst(op, i1, i2, i3, regs) {
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
  }
}

function task1() {
  let c = 0;

  samples.forEach(({ before, instr, after }) => {
    const selected = ops.filter((op) => {
      const a = runInst(op, instr[1], instr[2], instr[3], before);
      return `${a[0]};${a[1]};${a[2]};${a[3]}` === after.join(";");
    });
    if (selected.length >= 3) c++;
  });

  return c;
}

function task2() {
  const opcodes = [];
  for (let i = 0; i < 16; i++) opcodes.push(ops);

  samples.forEach(({ before, instr, after }) => {
    const selected = opcodes[instr[0]].filter((op) => {
      const a = runInst(op, instr[1], instr[2], instr[3], before);
      return `${a[0]};${a[1]};${a[2]};${a[3]}` === after.join(";");
    });
    opcodes[instr[0]] = selected;
  });

  const resolved = {};
  while (Object.keys(resolved).length !== opcodes.length) {
    const instCode = opcodes.findIndex((ops) => ops.length === 1);
    if (instCode === -1) break;
    resolved[instCode] = opcodes[instCode][0];
    for (let i = 0; i < opcodes.length; i++) opcodes[i] = opcodes[i].filter((op) => op !== resolved[instCode]);
  }

  let regs = [0, 0, 0, 0];
  input[1].split("\n").forEach((line) => {
    const instr = line.split(" ").map((a) => Number(a));
    regs = runInst(resolved[instr[0]], instr[1], instr[2], instr[3], regs);
  });

  return regs[0];
}

console.log(task1());
console.log(task2());
