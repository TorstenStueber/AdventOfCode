const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").split("\n");

function isPrime(a) {
  for (let i = 2, s = Math.sqrt(a); i <= s; i++) {
    if (a % i === 0) return false;
  }
  return true;
}

function exec(state, input) {
  let inst = input[state.pc];
  const ops = inst
    .slice(4)
    .split(" ")
    .map((a) => (/-?\d+/.exec(a) ? Number(a) : state.regs[a] ?? 0));

  const leftOps = inst.slice(4).split(" ");

  switch (inst.slice(0, 3)) {
    case "set":
      state.regs[leftOps[0]] = ops[1];
      state.pc++;
      break;
    case "sub":
      state.regs[leftOps[0]] = ops[0] - ops[1];
      state.pc++;
      break;
    case "mul":
      state.regs[leftOps[0]] = ops[0] * ops[1];
      state.pc++;
      break;
    case "nop":
      state.pc++;
      break;
    case "pri":
      state.regs.f = isPrime(state.regs.b) ? 1 : 0;
      state.pc++;
      break;
    case "jnz":
      if (ops[0] !== 0) {
        state.pc += ops[1];
      } else {
        state.pc++;
      }
      break;
  }
}

function task1() {
  const state = { pc: 0, regs: {} };
  let c = 0;
  while (state.pc < input.length) {
    if (input[state.pc].startsWith("mul")) c++;
    exec(state, input);
  }

  return c;
}

const isPrimeNumber = `set f 1
set d 2
set e 2
set g d
mul g e
sub g b
jnz g 2
set f 0
sub e -1
set g e
sub g b
jnz g -8
sub d -1
set g d
sub g b
jnz g -13`;

function task2() {
  let input2 = [...input];
  for (let i = 0; i < input.length; i++) {
    if (input.slice(i).join("\n").startsWith(isPrimeNumber)) {
      for (let j = 0; j < isPrimeNumber.split("\n").length; j++) {
        input2[i + j] = j === 0 ? "pri" : "nop";
      }
    }
  }

  const state = { pc: 0, regs: { a: 1 } };
  while (state.pc < input2.length) {
    exec(state, input2);
  }

  return state.regs.h;
}

console.log(task1());
console.log(task2());
