const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("");

function step(numbers) {
  return numbers.map((_, i) => {
    const r = numbers.reduce((acc, n, j) => acc + n * [0, 1, 0, -1][Math.floor((j + 1) / (i + 1)) % 4], 0) % 10;
    return Math.abs(r);
  });
}

function task1() {
  let n = input;
  for (let i = 0; i < 100; i++) {
    n = step(n);
  }

  return n.join("").slice(0, 8);
}

function task2() {
  let k = Number(input.join("").slice(0, 7));
  let n = [];
  for (let i = k; i < 10000 * input.length; i++) {
    n.push(input[i % input.length]);
  }

  for (let i = 0; i < 100; i++) {
    let s = 0;
    for (let j = n.length - 1; j >= 0; j--) {
      s += Number(n[j]);
      n[j] = Math.abs(s % 10);
    }
  }

  return n.join("").slice(0, 8);
}

console.log(task1());
console.log(task2());
