const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function task1() {
  let acc = 0;
  let pc = 0;
  const ex = [];

  while (true) {
    if (ex.indexOf(pc) >= 0) return acc;
    ex.push(pc);
    if (input[pc].startsWith("acc")) {
      acc += Number(input[pc].slice(4));
      pc++;
    } else if (input[pc].startsWith("jmp")) {
      pc += Number(input[pc].slice(4));
    } else {
      pc++;
    }
  }
}

function ex(pos) {
  const instr = [...input];
  if (instr[pos].startsWith("acc")) return false;
  else if (instr[pos].startsWith("jmp")) instr[pos] = `nop ${instr[pos].slice(4)}`;
  else if (instr[pos].startsWith("nop")) instr[pos] = `jmp ${instr[pos].slice(4)}`;
  let acc = 0;
  let pc = 0;
  const ex = [];

  while (true) {
    if (pc >= instr.length) return acc;
    if (ex.indexOf(pc) >= 0) return false;
    ex.push(pc);
    if (instr[pc].startsWith("acc")) {
      acc += Number(instr[pc].slice(4));
      pc++;
    } else if (instr[pc].startsWith("jmp")) {
      pc += Number(instr[pc].slice(4));
    } else {
      pc++;
    }
  }
}

function task2() {
  for (let i = 0; i < input.length; i++) {
    const r = ex(i);
    if (r !== false) return r;
  }
}

console.log(task1());
console.log(task2());
