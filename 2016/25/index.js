const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function run(a, program, output, maxOutLength) {
  let pc = 0;
  let reg = { a, b: 0, c: 0, d: 0 };
  while (pc < program.length) {
    const inst = program[pc];
    switch (inst.slice(0, 3)) {
      case "cpy":
        {
          const [x, y] = inst.slice(4).split(" ");
          if (!/-?\d+/.exec(y)) {
            const valX = /-?\d+/.exec(x) ? Number(x) : reg[x];
            reg[y] = valX;
          }
        }
        pc++;
        break;
      case "inc":
        if (!/-?\d+/.exec(inst.slice(4))) {
          reg[inst.slice(4)]++;
        }
        pc++;
        break;
      case "dec":
        if (!/-?\d+/.exec(inst.slice(4))) {
          reg[inst.slice(4)]--;
        }
        pc++;
        break;
      case "out":
        {
          const out = /-?\d+/.exec(inst.slice(4)) ? Number(inst.slice(4)) : reg[inst.slice(4)];
          output.push(out);
          if (output.length > maxOutLength) return;
        }
        pc++;
        break;
      case "jnz":
        {
          const [x, y] = inst.slice(4).split(" ");
          const valX = /-?\d+/.exec(x) ? Number(x) : reg[x];
          const valY = /-?\d+/.exec(y) ? Number(y) : reg[y];
          if (valX !== 0) {
            pc += valY;
          } else {
            pc++;
          }
        }
        break;
      case "tgl":
        {
          const pos = pc + (/-?\d+/.exec(inst.slice(4)) ? Number(inst.slice(4)) : reg[inst.slice(4)]);
          if (0 <= pos && pos < program.length) {
            const posInst = program[pos];
            let newInst;
            if (posInst.split(" ").length === 2) {
              if (posInst.startsWith("inc")) {
                newInst = `dec ${posInst.slice(4)}`;
              } else {
                newInst = `inc ${posInst.slice(4)}`;
              }
            } else {
              if (posInst.startsWith("jnz")) {
                newInst = `cpy ${posInst.slice(4)}`;
              } else {
                newInst = `jnz ${posInst.slice(4)}`;
              }
            }
            program[pos] = newInst;
          }
          pc++;
        }
        break;
    }
  }

  return reg.a;
}

function task1() {
  let a = 0;
  while (true) {
    const output = [];
    run(a, [...input], output, 36);
    if (output.every((a, i) => a === (i % 2 === 0 ? 0 : 1))) return a;
    a++;
  }
}

console.log(task1());
