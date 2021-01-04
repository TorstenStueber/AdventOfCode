const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n");

function task1() {
  let count = 0;
  let x = 0;
  input.forEach((line, index) => {
    if (line[(index * 3) % line.length] === "#") count++;
  });
  return count;
}

function test(yInc, xInc) {
  let x = 0;
  let y = 0;
  let count = 0;
  while (y < input.length) {
    if (input[y][x % input[y].length] === "#") count++;
    x += xInc;
    y += yInc;
  }
  return count;
}

function task2() {
  return test(1, 1) * test(1, 3) * test(1, 5) * test(1, 7) * test(2, 1);
}

console.log(task1());
console.log(task2());
