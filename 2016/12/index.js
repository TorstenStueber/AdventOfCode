const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function run(c) {
  let pc = 0;
  let reg = { a: 0, b: 0, c, d: 0 };
  while (pc < input.length) {
    const inst = input[pc];
    switch (inst.slice(0, 3)) {
      case "cpy":
        {
          const [x, y] = inst.slice(4).split(" ");
          if (/\d+/.exec(x)) {
            reg[y] = Number(x);
          } else {
            reg[y] = reg[x];
          }
        }
        pc++;
        break;
      case "inc":
        reg[inst.slice(4)]++;
        pc++;
        break;
      case "dec":
        reg[inst.slice(4)]--;
        pc++;
        break;
      case "jnz": {
        const [x, y] = inst.slice(4).split(" ");
        if (reg[x] !== 0) {
          pc += Number(y);
        } else {
          pc++;
        }
      }
    }
  }

  return reg.a;
}

function task1() {
  return run(0);
}

function task2() {
  return run(1);
}

console.log(task1());
console.log(task2());
