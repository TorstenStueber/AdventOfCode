const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n\n");

function task1() {
  return input.reduce((acc, g) => {
    g = g.replace(/\s/g, "");
    const s = new Set();
    g.split("").forEach((a) => s.add(a));
    return acc + Array.from(s).length;
  }, 0);
}

function task2() {
  return input.reduce((acc, g) => {
    g = g.split("\n");
    const s = new Set();
    g[0].split("").forEach((a) => s.add(a));
    return acc + Array.from(s).filter((s) => g.every((line) => line.indexOf(s) >= 0)).length;
  }, 0);
}

console.log(task1());
console.log(task2());
