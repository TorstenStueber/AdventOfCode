const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function task1() {
  const transpose = [];
  for (let i = 0; i < input[0].length; i++) {
    const line = [];
    input.forEach((k) => line.push(k[i]));
    transpose.push(line);
  }
  return transpose.reduce((a, line) => {
    const f = {};
    line.forEach((s) => {
      if (f[s] === undefined) f[s] = 0;
      f[s]++;
    });
    return a + Object.keys(f).sort((a, b) => f[b] - f[a])[0];
  }, "");
}

function task2() {
  const transpose = [];
  for (let i = 0; i < input[0].length; i++) {
    const line = [];
    input.forEach((k) => line.push(k[i]));
    transpose.push(line);
  }
  return transpose.reduce((a, line) => {
    const f = {};
    line.forEach((s) => {
      if (f[s] === undefined) f[s] = 0;
      f[s]++;
    });
    return a + Object.keys(f).sort((a, b) => f[a] - f[b])[0];
  }, "");
}

console.log(task1());
console.log(task2());
