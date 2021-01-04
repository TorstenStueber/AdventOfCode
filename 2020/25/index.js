const { strict } = require("assert");
const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((a) => Number(a));

function step(current, subjectNumber) {
  return (current * subjectNumber) % 20201227;
}

function findLoop(l, subjectNumber) {
  let current = 1;
  let count = 0;
  while (current !== l) {
    count++;
    current = step(current, subjectNumber);
  }
  return count;
}

function task1() {
  let l1 = findLoop(input[0], 7);
  let l2 = findLoop(input[1], 7);

  let current = 1;
  while (l2-- > 0) current = step(current, input[0]);

  return current;
}

console.log(task1());
