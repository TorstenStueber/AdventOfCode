const { readFileSync, chmod } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function execute(a) {
  let pc = 0;
  let reg = { a, b: 0 };
  while (pc < input.length) {
    if (input[pc].startsWith("hlf")) {
      reg[input[pc].slice(4)] = Math.floor(reg[input[pc].slice(4)] / 2);
      pc++;
    } else if (input[pc].startsWith("tpl")) {
      reg[input[pc].slice(4)] *= 3;
      pc++;
    } else if (input[pc].startsWith("inc")) {
      reg[input[pc].slice(4)]++;
      pc++;
    } else if (input[pc].startsWith("jmp")) {
      pc += Number(input[pc].slice(4));
    } else if (input[pc].startsWith("jie")) {
      const r = reg[input[pc].split(",")[0].slice(4)];
      const o = Number(input[pc].split(", ")[1]);
      if (r % 2 === 0) {
        pc += o;
      } else {
        pc++;
      }
    } else if (input[pc].startsWith("jio")) {
      const r = reg[input[pc].split(",")[0].slice(4)];
      const o = Number(input[pc].split(", ")[1]);
      if (r === 1) {
        pc += o;
      } else {
        pc++;
      }
    }
  }

  return reg.b;
}

function task1() {
  return execute(0);
}

function task2() {
  return execute(1);
}

console.log(task1());
console.log(task2());
