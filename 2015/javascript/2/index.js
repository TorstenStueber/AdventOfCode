const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function task1() {
  return input.reduce((a, b) => {
    const [w, h, l] = b.split("x").map((a) => Number(a));
    const area = 2 * (w * h + w * l + h * l) + Math.min(w * h, w * l, h * l);
    return a + area;
  }, 0);
}

function task2() {
  return input.reduce((a, b) => {
    const [w, h, l] = b.split("x").map((a) => Number(a));
    const area = w * h * l + 2 * Math.min(w + h, w + l, h + l);
    return a + area;
  }, 0);
}

console.log(task1());
console.log(task2());
