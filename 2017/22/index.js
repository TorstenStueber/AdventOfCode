const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .split("\n")
  .map((b) => b.split(""));

const dirs = [
  [0, -1],
  [1, 0],
  [0, 1],
  [-1, 0],
];

function task1() {
  let dir = 0;
  let x = 0;
  let y = 0;
  let state = {};

  input.forEach((line, y) => {
    line.forEach((a, x) => {
      if (a === "#") state[`${x - (line.length - 1) / 2};${y - (input.length - 1) / 2}`] = "#";
    });
  });

  let c = 0;
  for (let i = 0; i < 10000; i++) {
    const key = `${x};${y}`;
    if (state[key] === "#") {
      dir = (dir + 1) % 4;
      state[key] = ".";
    } else {
      dir = (dir + 3) % 4;
      state[key] = "#";
      c++;
    }
    x += dirs[dir][0];
    y += dirs[dir][1];
  }

  return c;
}

function task2() {
  let dir = 0;
  let x = 0;
  let y = 0;
  let state = {};

  input.forEach((line, y) => {
    line.forEach((a, x) => {
      if (a === "#") state[`${x - (line.length - 1) / 2};${y - (input.length - 1) / 2}`] = "#";
    });
  });

  let c = 0;
  for (let i = 0; i < 10000000; i++) {
    const key = `${x};${y}`;
    if (state[key] === "#") {
      dir = (dir + 1) % 4;
      state[key] = "F";
    } else if (state[key] === "F") {
      dir = (dir + 2) % 4;
      state[key] = ".";
    } else if (state[key] === "." || state[key] === undefined) {
      dir = (dir + 3) % 4;
      state[key] = "W";
    } else {
      state[key] = "#";
      c++;
    }
    x += dirs[dir][0];
    y += dirs[dir][1];
  }

  return c;
}

console.log(task1());
console.log(task2());
