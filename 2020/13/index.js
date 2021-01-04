const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function task1() {
  const t = Number(input[0]);
  let min = Infinity;
  let bus;
  input[1].split(",").forEach((l) => {
    if (l === "x") return;
    const time = Number(l);
    const left = Math.ceil(t / time) * time - t;
    if (left < min) {
      min = left;
      bus = l;
    }
  });

  return bus * min;
}

function euclid(a, b) {
  while (b !== 0n) {
    x = a % b;
    a = b;
    b = x;
  }
  return a;
}

function kgV(a, b) {
  return (a * b) / euclid(a, b);
}

function task2() {
  let k = 1n;
  let last = 0n;
  input[1].split(",").forEach((l, diff) => {
    if (l === "x") return;
    const time = BigInt(Number(l));
    while (diff > 0 && (last + BigInt(diff)) % time !== 0n) {
      last += k;
    }
    k = kgV(k, time);
  });

  return last;
}

console.log(task1());
console.log(task2());
