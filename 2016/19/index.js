const { readFileSync } = require("fs");
const { join } = require("path");

const input = Number(readFileSync(join(__dirname, "input.txt"), "utf-8").trim());

function task1() {
  const elves = [];
  for (let i = 0; i < input; i++) elves.push(true);
  let left = input;
  let cursor = 0;
  while (left > 1) {
    cursor++;
    if (cursor === elves.length) cursor = 0;
    while (elves[cursor] === false) {
      cursor++;
      if (cursor === elves.length) cursor = 0;
    }
    elves[cursor] = false;
    left--;
    while (elves[cursor] === false) {
      cursor++;
      if (cursor === elves.length) cursor = 0;
    }
  }

  return cursor + 1;
}

function task2() {
  const elves = [];
  for (let i = 0; i < input; i++) elves.push(true);
  let left = input;
  let cursor = Math.floor(elves.length / 2);
  while (left > 1) {
    elves[cursor] = false;
    left--;

    let skip = left % 2 ? 1 : 2;
    while (skip > 0) {
      cursor = (cursor + 1) % elves.length;
      while (elves[cursor] === false) {
        cursor = (cursor + 1) % elves.length;
      }
      skip--;
    }
  }
  return cursor + 1;
}

console.log(task1());
console.log(task2());
