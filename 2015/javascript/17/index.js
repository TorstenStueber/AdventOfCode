const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((a) => Number(a));

function task1() {
  let c = 0;

  for (let i = 0; i < 2 ** input.length; i++) {
    const k = i.toString(2).padStart(input.length, "0");
    if (input.reduce((a, b, j) => a + Number(k[j]) * b, 0) === 150) c++;
  }

  return c;
}

function task2() {
  let c = 0;

  let min = Infinity;
  for (let i = 0; i < 2 ** input.length; i++) {
    const k = i.toString(2).padStart(input.length, "0");
    if (input.reduce((a, b, j) => a + Number(k[j]) * b, 0) === 150) {
      min = Math.min(min, k.split("").filter((a) => a === "1").length);
    }
  }

  for (let i = 0; i < 2 ** input.length; i++) {
    const k = i.toString(2).padStart(input.length, "0");
    if (k.split("").filter((a) => a === "1").length !== min) continue;
    if (input.reduce((a, b, j) => a + Number(k[j]) * b, 0) === 150) c++;
  }

  return c;
}

console.log(task1());
console.log(task2());
