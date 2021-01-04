const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((line) => {
    const [_, a, b, c] = /\s*(\d+)\s+(\d+)\s+(\d+)/.exec(line);
    return { a: Number(a), b: Number(b), c: Number(c) };
  });

function task1() {
  return input.reduce((x, { a, b, c }) => x + (a + b > c && a + c > b && b + c > a ? 1 : 0), 0);
}

const input2 = [];
for (let i = 0; i < input.length; i += 3) {
  input2.push({ a: input[i].a, b: input[i + 1].a, c: input[i + 2].a });
  input2.push({ a: input[i].b, b: input[i + 1].b, c: input[i + 2].b });
  input2.push({ a: input[i].c, b: input[i + 1].c, c: input[i + 2].c });
}

function task2() {
  return input2.reduce((x, { a, b, c }) => x + (a + b > c && a + c > b && b + c > a ? 1 : 0), 0);
}

console.log(task1());
console.log(task2());
