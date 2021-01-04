const { readFileSync } = require("fs");
const { join } = require("path");

const input = Number(readFileSync(join(__dirname, "input.txt"), "utf-8").trim());

const dirs = [
  [0, 1],
  [1, 0],
  [0, -1],
  [-1, 0],
];

function task1() {
  let x = 0;
  let y = 0;
  let current = 1;
  const used = [];
  let dir = 0;
  while (true) {
    used[`${x};${y}`] = current;
    const nextDir = dirs[(dir + 1) % 4];
    if (used[`${x + nextDir[0]};${y + nextDir[1]}`] === undefined) {
      dir = (dir + 1) % 4;
    }
    x += dirs[dir][0];
    y += dirs[dir][1];
    current++;

    if (current === input) return Math.abs(x) + Math.abs(y);
  }
}

function task2() {
  let x = 0;
  let y = 0;
  let current = 1;
  const used = { "0;0": 1 };
  let dir = 0;
  while (true) {
    let c = 0;
    for (let xx = -1; xx <= 1; xx++) {
      for (let yy = -1; yy <= 1; yy++) {
        c += used[`${x + xx};${y + yy}`] ?? 0;
      }
    }
    used[`${x};${y}`] = c;
    if (c > input) return c;

    const nextDir = dirs[(dir + 1) % 4];
    if (used[`${x + nextDir[0]};${y + nextDir[1]}`] === undefined) {
      dir = (dir + 1) % 4;
    }
    x += dirs[dir][0];
    y += dirs[dir][1];
  }
}

console.log(task1());
console.log(task2());
