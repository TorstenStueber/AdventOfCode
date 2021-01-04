const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split(" ")
  .map((a) => Number(a));

function task1() {
  let sum = 0;
  function traverse(start) {
    const nodes = input[start];
    const meta = input[start + 1];
    start += 2;
    for (let i = 0; i < nodes; i++) {
      start = traverse(start);
    }
    for (let i = 0; i < meta; i++) {
      sum += input[start++];
    }

    return start;
  }

  traverse(0);

  return sum;
}

function task2() {
  function traverse(start) {
    const nodes = input[start];
    const meta = input[start + 1];
    start += 2;

    const values = [];
    for (let i = 0; i < nodes; i++) {
      const result = traverse(start);
      start = result.start;
      values.push(result.value);
    }

    const metadata = [];
    for (let i = 0; i < meta; i++) {
      metadata.push(input[start++]);
    }

    let value;
    if (nodes === 0) {
      value = metadata.reduce((a, b) => a + b, 0);
    } else {
      value = metadata.reduce((a, b) => a + (values[b - 1] ?? 0), 0);
    }

    return { start, value };
  }

  return traverse(0).value;
}

console.log(task1());
console.log(task2());
